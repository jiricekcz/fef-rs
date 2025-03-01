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

/// Metadata record with identifier reserved for official use by the FEF specification.
///
/// These keys are reserved for future use by the FEF specification.
/// They may be defined later. If this record is encountered, it should be ignored - either the data is misformed or the library is outdated.
/// Make sure that encountering this record does not cause a failure in your application - applications should guarantee forward compatibility when
/// encountering unknown metadata keys.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OfficialReservedMetadataRecordObj {
    pub(crate) identifier: u32,
    data: Vec<u8>,
}

impl Sealed for OfficialReservedMetadataRecordObj {}

impl MetadataRecordObj for OfficialReservedMetadataRecordObj {
    fn token(&self) -> Result<MetadataToken, MetadataTokenError> {
        Ok(MetadataToken::ReservedOfficial(self.identifier))
    }
    fn byte_length(&self) -> usize {
        self.data.len()
    }
}

impl OfficialReservedMetadataRecordObj {
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

impl<W: ?Sized + Write> WriteTo<W> for OfficialReservedMetadataRecordObj {
    type WriteError = MetadataRecordWriteError;

    /// Writes the metadata record to a writer.
    ///
    /// This method should rarely be used, as it creates a situation where the library is on purpose creating a record that is not defined in the specification.
    /// This is for example useful, if you want to copy a record or convert it from a different format - this way you don't need to know the exact structure of the record.
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

impl Into<ReservedMetadataRecord> for OfficialReservedMetadataRecordObj {
    fn into(self) -> ReservedMetadataRecord {
        ReservedMetadataRecord::Official(self)
    }
}
