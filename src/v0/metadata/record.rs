use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        raw::VariableLengthEnum,
        tokens::{error::MetadataTokenError, MetadataToken},
        traits::{ReadFrom, WriteTo},
    },
};

use super::{
    error::{MetadataRecordReadError, MetadataRecordWriteError},
    traits::MetadataRecordObj,
    NameMetadataRecordObj, ReservedMetadataRecord, UnknownMetadataRecordObj,
    VariableNameMetadataRecordObj,
};
/// A [metadata record](https://github.com/jiricekcz/fef-specification/blob/main/metadata/Metadata.md#metadata-keys).
///
/// All library-defined metadata records are represented by this enum. There are also catch all variants for unknown metadata records and reserved metadata records.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum MetadataRecord {
    Name(NameMetadataRecordObj),
    VariableName(VariableNameMetadataRecordObj),
    Reserved(ReservedMetadataRecord),
    Unknown(UnknownMetadataRecordObj),
}

impl Sealed for MetadataRecord {}

impl<R: Read + ?Sized> ReadFrom<R> for MetadataRecord {
    type ReadError = MetadataRecordReadError;

    /// Reads a metadata record from a reader.
    ///
    /// It reads the metadata token and then reads the metadata record based on the token.
    ///
    /// # Example
    ///
    /// Reading a known and unknown metadata record:
    /// ```rust
    /// # use fef::v0::metadata::MetadataRecord;
    /// # use fef::v0::metadata::NameMetadataRecordObj;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// # use fef::v0::metadata::UnknownMetadataRecordObj;
    /// # use fef::v0::metadata::ReservedMetadataRecord;
    /// # use fef::v0::traits::ReadFrom;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let data = vec![
    ///     // Known metadata record - Name
    ///     0x01, // Metadata token
    ///     0x08, // Length of the name record
    ///     0x07,  // Length of the name string
    ///     b'F', b'o', b'r', b'm', b'u', b'l', b'a', // Name string
    ///     // Unknown reserved metadata record
    ///     0x1F, // Metadata token
    ///     0x08, // Length of the record
    ///     0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21, 0x21, 0x21, // Record data
    ///     // Unknown not reserved metadata record
    ///     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, // Metadata token
    ///     0x02, // Length of the record
    ///     0x57, 0x6F // Record data
    /// ];
    /// let mut reader = &mut data.as_slice();
    ///
    /// let record1 = MetadataRecord::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// assert_eq!(record1, MetadataRecord::Name(NameMetadataRecordObj::new("Formula".to_string())));
    ///
    /// let record2 = MetadataRecord::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// assert!(matches!(record2, MetadataRecord::Reserved(ReservedMetadataRecord::Official(_))));
    ///
    /// let record3 = MetadataRecord::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// assert!(matches!(record3, MetadataRecord::Unknown(_)));
    /// # Ok(())
    /// # }
    /// ```
    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let identifier = VariableLengthEnum::read_from(reader, configuration)?;
        let token = match MetadataToken::try_from(identifier) {
            Ok(token) => token,
            Err(error) => match error {
                MetadataTokenError::IdentifierNotRecognized { identifier } => {
                    let record =
                        UnknownMetadataRecordObj::read_from(reader, configuration, identifier)?;
                    return Ok(MetadataRecord::Unknown(record));
                }
                _ => return Err(error.into()),
            },
        };
        match token {
            MetadataToken::Name => {
                let record = NameMetadataRecordObj::read_from(reader, configuration)?;
                Ok(MetadataRecord::Name(record))
            }
            MetadataToken::VariableName => {
                let record = VariableNameMetadataRecordObj::read_from(reader, configuration)?;
                Ok(MetadataRecord::VariableName(record))
            }
            MetadataToken::ReservedOfficial(_) => {
                let record: ReservedMetadataRecord =
                    ReservedMetadataRecord::read_from(reader, configuration, token)?;
                Ok(MetadataRecord::Reserved(record))
            }
            MetadataToken::ReservedThirdParty(_) => {
                let record = ReservedMetadataRecord::read_from(reader, configuration, token)?;
                Ok(MetadataRecord::Reserved(record))
            }
            MetadataToken::ReservedCustom(_) => {
                let record = ReservedMetadataRecord::read_from(reader, configuration, token)?;
                Ok(MetadataRecord::Reserved(record))
            }
        }
    }
}

macro_rules! write_metadata_record {
    ($record:ident, $writer:ident, $configuration:ident) => {{
        let identifier: VariableLengthEnum = match $record.token() {
            Err(err) => match err {
                MetadataTokenError::IdentifierTooLarge { identifier } => identifier,
                MetadataTokenError::IdentifierNotRecognized { identifier } => identifier,
            },
            Ok(token) => <MetadataToken as Into<usize>>::into(token).into(),
        };
        identifier.write_to($writer, $configuration)?;
        $record.write_to($writer, $configuration)?;
    }};
}

impl<W: ?Sized + Write> WriteTo<W> for MetadataRecord {
    type WriteError = MetadataRecordWriteError;

    /// Writes the metadata record to a writer.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fef::v0::metadata::MetadataRecord;
    /// # use fef::v0::metadata::NameMetadataRecordObj;
    /// # use fef::v0::metadata::VariableNameMetadataRecordObj;
    /// # use fef::v0::metadata::ReservedMetadataRecord;
    /// # use fef::v0::metadata::CustomReservedMetadataRecordObj;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::WriteTo;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let record1 = MetadataRecord::Name(NameMetadataRecordObj::new("Formula".to_string()));
    /// let record2 = MetadataRecord::VariableName(VariableNameMetadataRecordObj::new("x".to_string(), 1.into()));
    ///
    /// let record3_inner = CustomReservedMetadataRecordObj::new(0x100100, vec![0x12, 0x34])?;
    /// let record3 = MetadataRecord::Reserved(ReservedMetadataRecord::Custom(record3_inner));
    ///
    /// let mut writer: Vec<u8> = Vec::new();
    ///
    /// record1.write_to(&mut writer, &DEFAULT_CONFIG)?;
    /// record2.write_to(&mut writer, &DEFAULT_CONFIG)?;
    /// record3.write_to(&mut writer, &DEFAULT_CONFIG)?;
    ///
    /// assert_eq!(writer, vec![
    ///     // Record 1
    ///     0x01, // Metadata token
    ///     0x08, // Length of the name record
    ///     0x07,  // Length of the name string
    ///     b'F', b'o', b'r', b'm', b'u', b'l', b'a', // Name string
    ///     // Record 2
    ///     0x02, // Metadata token
    ///     0x03, // Length of the record
    ///     0x01, // Variable identifier
    ///     0x01, // Length of the string
    ///     b'x', // Name
    ///     // Record 3
    ///     0xC0, 0x82, 0x00, // Metadata token
    ///     0x02, // Length of the record
    ///     0x12, 0x34, // Record data
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        match self {
            MetadataRecord::Name(record) => write_metadata_record!(record, writer, configuration),
            MetadataRecord::VariableName(record) => {
                write_metadata_record!(record, writer, configuration)
            }
            MetadataRecord::Reserved(record) => {
                write_metadata_record!(record, writer, configuration)
            }
            MetadataRecord::Unknown(record) => {
                write_metadata_record!(record, writer, configuration)
            }
        };
        Ok(())
    }
}

impl MetadataRecord {
    pub(crate) fn byte_length(&self) -> usize {
        match self {
            MetadataRecord::Name(record) => record.byte_length(),
            MetadataRecord::VariableName(record) => record.byte_length(),
            MetadataRecord::Reserved(record) => record.byte_length(),
            MetadataRecord::Unknown(record) => record.byte_length(),
        }
    }
}
