mod custom;
mod official;
mod third_party;

use std::io::{Read, Write};

pub use custom::CustomReservedMetadataRecordObj;
pub use official::OfficialReservedMetadataRecordObj;
pub use third_party::ThirdPartyReservedMetadataRecordObj;

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        metadata::{
            error::{MetadataRecordReadError, MetadataRecordWriteError},
            traits::MetadataRecordObj,
        },
        tokens::{error::MetadataTokenError, MetadataToken},
        traits::WriteTo,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReservedMetadataRecord {
    Official(OfficialReservedMetadataRecordObj),
    Custom(CustomReservedMetadataRecordObj),
    ThirdParty(ThirdPartyReservedMetadataRecordObj),
}
impl Sealed for ReservedMetadataRecord {}

impl MetadataRecordObj for ReservedMetadataRecord {
    fn token(&self) -> Result<MetadataToken, MetadataTokenError> {
        match self {
            ReservedMetadataRecord::Official(record) => record.token(),
            ReservedMetadataRecord::Custom(record) => record.token(),
            ReservedMetadataRecord::ThirdParty(record) => record.token(),
        }
    }
    fn byte_length(&self) -> usize {
        match self {
            ReservedMetadataRecord::Official(record) => record.byte_length(),
            ReservedMetadataRecord::Custom(record) => record.byte_length(),
            ReservedMetadataRecord::ThirdParty(record) => record.byte_length(),
        }
    }
}

impl ReservedMetadataRecord {
    pub(crate) fn read_from<C: ?Sized + Config, R: ?Sized + Read>(
        reader: &mut R,
        configuration: &C,
        identifier: MetadataToken,
    ) -> Result<Self, MetadataRecordReadError> {
        match identifier {
            MetadataToken::ReservedOfficial(identifier) => {
                let record = OfficialReservedMetadataRecordObj::read_from(
                    reader,
                    configuration,
                    identifier,
                )?;
                Ok(ReservedMetadataRecord::Official(record))
            }
            MetadataToken::ReservedThirdParty(identifier) => {
                let record = ThirdPartyReservedMetadataRecordObj::read_from(
                    reader,
                    configuration,
                    identifier,
                )?;
                Ok(ReservedMetadataRecord::ThirdParty(record))
            }
            MetadataToken::ReservedCustom(identifier) => {
                let record =
                    CustomReservedMetadataRecordObj::read_from(reader, configuration, identifier)?;
                Ok(ReservedMetadataRecord::Custom(record))
            }
            _ => unreachable!(),
        }
    }
}

impl<W: ?Sized + Write> WriteTo<W> for ReservedMetadataRecord {
    type WriteError = MetadataRecordWriteError;
    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        match self {
            ReservedMetadataRecord::Official(record) => record.write_to(writer, configuration),
            ReservedMetadataRecord::Custom(record) => record.write_to(writer, configuration),
            ReservedMetadataRecord::ThirdParty(record) => record.write_to(writer, configuration),
        }
    }
}
