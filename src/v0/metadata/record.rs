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

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum MetadataRecord {
    Name(NameMetadataRecordObj),
    VariableName(VariableNameMetadataRecordObj),
    Reserved(ReservedMetadataRecord),
    Unknown(UnknownMetadataRecordObj),
}

impl Sealed for MetadataRecord {}

impl<R: Read + ?Sized> ReadFrom<R> for MetadataRecord {
    type ReadError = MetadataRecordReadError;
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
