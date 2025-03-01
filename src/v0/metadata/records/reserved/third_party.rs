use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        metadata::{
            error::{MetadataRecordReadError, MetadataRecordWriteError},
            traits::MetadataRecordObj,
        },
        raw::VariableLengthEnum,
        tokens::{error::MetadataTokenError, MetadataToken},
        traits::{ReadFrom, WriteTo},
    },
};

use super::ReservedMetadataRecord;
/// Metadata record with identifier reserved for third-party extensions to the FEF specification.
///
/// In general, handling of third-party metadata records is determined by the third-party extension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThirdPartyReservedMetadataRecordObj {
    pub(crate) identifier: u32,
    data: Vec<u8>,
}

impl Sealed for ThirdPartyReservedMetadataRecordObj {}

impl MetadataRecordObj for ThirdPartyReservedMetadataRecordObj {
    fn token(&self) -> Result<MetadataToken, MetadataTokenError> {
        Ok(MetadataToken::ReservedOfficial(self.identifier))
    }
    fn byte_length(&self) -> usize {
        self.data.len()
    }
}

impl ThirdPartyReservedMetadataRecordObj {
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

impl<W: ?Sized + Write> WriteTo<W> for ThirdPartyReservedMetadataRecordObj {
    type WriteError = MetadataRecordWriteError;

    /// Writes the metadata record to a writer.
    ///
    /// Should generally be only used by the third-party extension that defined the metadata record.
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

impl Into<ReservedMetadataRecord> for ThirdPartyReservedMetadataRecordObj {
    fn into(self) -> ReservedMetadataRecord {
        ReservedMetadataRecord::ThirdParty(self)
    }
}
