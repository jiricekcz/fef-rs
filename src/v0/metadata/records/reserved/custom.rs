use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        metadata::{
            error::{
                MetadataIdentifierOutOfRangeError, MetadataRecordReadError,
                MetadataRecordWriteError,
            },
            traits::MetadataRecordObj,
        },
        raw::VariableLengthEnum,
        tokens::{error::MetadataTokenError, MetadataToken},
        traits::{ReadFrom, WriteTo},
    },
};

use super::ReservedMetadataRecord;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Metadata record with identifier reserved for custom use by any implementor.
///
/// # Example
///
/// Creating a custom metadata record:
/// ```rust
/// # use fef::v0::metadata::CustomReservedMetadataRecordObj;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let record = CustomReservedMetadataRecordObj::new(0x100100, vec![0x12, 0x34])?;
///
/// assert_eq!(record.identifier(), 0x100100);
/// assert_eq!(record.data(), &[0x12, 0x34]);
/// # Ok(())
/// # }
/// ```
pub struct CustomReservedMetadataRecordObj {
    identifier: u32,
    data: Vec<u8>,
}

impl Sealed for CustomReservedMetadataRecordObj {}

impl MetadataRecordObj for CustomReservedMetadataRecordObj {
    fn token(&self) -> Result<MetadataToken, MetadataTokenError> {
        Ok(MetadataToken::ReservedOfficial(self.identifier))
    }
    fn byte_length(&self) -> usize {
        self.data.len()
    }
}

impl CustomReservedMetadataRecordObj {
    /// Reads a custom reserved metadata record from a reader.
    ///
    /// This method expects the identifier for this record is already read (it must be in order to know which type of record to read), but requires this identifier to be passed as an argument.
    pub(crate) fn read_from<C: ?Sized + Config, R: ?Sized + Read>(
        reader: &mut R,
        configuration: &C,
        identifier: u32,
    ) -> Result<Self, MetadataRecordReadError> {
        let length: usize = VariableLengthEnum::read_from(reader, configuration)?.try_into()?;
        let mut data = Vec::with_capacity(length);
        reader.take(length as u64).read_to_end(&mut data)?;
        Ok(Self { identifier, data })
    }
}

impl<W: ?Sized + Write> WriteTo<W> for CustomReservedMetadataRecordObj {
    type WriteError = MetadataRecordWriteError;
    /// Writes the custom reserved metadata record to a writer.
    ///
    /// This method writes the length of the data and then the data itself. It does not write the identifier.
    ///
    /// # Example
    /// ```rust
    /// # use fef::v0::metadata::CustomReservedMetadataRecordObj;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::WriteTo;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let record = CustomReservedMetadataRecordObj::new(0x100100, vec![0x12, 0x34])?;
    /// let mut writer: Vec<u8> = Vec::new();
    /// record.write_to(&mut writer, &DEFAULT_CONFIG)?;
    /// assert_eq!(writer, vec![0x02, 0x12, 0x34]);
    /// # Ok(())
    /// # }
    /// ```
    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        VariableLengthEnum::from(self.data.len()).write_to(writer, configuration)?;
        writer.write_all(&self.data)?;
        Ok(())
    }
}

impl CustomReservedMetadataRecordObj {
    /// Creates a new custom reserved metadata record.
    ///
    /// Checks, if the identifier is in the reserved range for custom records.
    /// If the identifier is out of range, returns an error.
    ///
    /// # Example
    ///
    /// Correctly creating a custom reserved metadata record:
    /// ```rust
    /// # use fef::v0::metadata::CustomReservedMetadataRecordObj;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let record = CustomReservedMetadataRecordObj::new(0x100100, vec![0x12, 0x34]);
    /// assert!(record.is_ok());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Incorrectly creating a custom reserved metadata record with out of range identifier:
    /// ```rust
    /// # use fef::v0::metadata::CustomReservedMetadataRecordObj;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let record = CustomReservedMetadataRecordObj::new(0x01, vec![0x12, 0x34]);
    /// assert!(record.is_err());
    /// # Ok(())
    /// # }
    pub fn new(identifier: u32, data: Vec<u8>) -> Result<Self, MetadataIdentifierOutOfRangeError> {
        if !(0x100000u32..0x1FFFFFu32).contains(&identifier) {
            return Err(MetadataIdentifierOutOfRangeError::custom_key(identifier));
        }
        Ok(Self { identifier, data })
    }

    /// Returns the identifier of the custom reserved metadata record.
    pub fn identifier(&self) -> u32 {
        self.identifier
    }

    /// Returns the data of the custom reserved metadata record.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Into<ReservedMetadataRecord> for CustomReservedMetadataRecordObj {
    fn into(self) -> ReservedMetadataRecord {
        ReservedMetadataRecord::Custom(self)
    }
}
