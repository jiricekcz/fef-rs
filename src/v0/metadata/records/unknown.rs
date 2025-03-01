use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        metadata::{
            error::{MetadataRecordReadError, MetadataRecordWriteError},
            traits::MetadataRecordObj,
            MetadataRecord,
        },
        raw::VariableLengthEnum,
        tokens::{error::MetadataTokenError, MetadataToken},
        traits::{ReadFrom, WriteTo},
    },
};
/// A generic metadata record with an unknown identifier.
///
/// Applications should generally ignore unknown metadata records.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownMetadataRecordObj {
    pub(crate) identifier: VariableLengthEnum,
    data: Vec<u8>,
}

impl Sealed for UnknownMetadataRecordObj {}

impl MetadataRecordObj for UnknownMetadataRecordObj {
    fn token(&self) -> Result<MetadataToken, MetadataTokenError> {
        Err(MetadataTokenError::IdentifierNotRecognized {
            identifier: self.identifier.to_owned(),
        })
    }
    fn byte_length(&self) -> usize {
        self.data.len()
    }
}

impl UnknownMetadataRecordObj {
    pub(crate) fn read_from<C: ?Sized + Config, R: ?Sized + Read>(
        reader: &mut R,
        configuration: &C,
        identifier: VariableLengthEnum,
    ) -> Result<Self, MetadataRecordReadError> {
        let length: usize = VariableLengthEnum::read_from(reader, configuration)?.try_into()?;
        let mut data = Vec::with_capacity(length);
        reader.take(length as u64).read_to_end(&mut data)?;
        Ok(Self { identifier, data })
    }
}

impl<W: ?Sized + Write> WriteTo<W> for UnknownMetadataRecordObj {
    type WriteError = MetadataRecordWriteError;

    /// Writes the metadata record to a writer.
    ///
    /// Only useful when copying metadata records or converting them from a different format.
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

impl Into<MetadataRecord> for UnknownMetadataRecordObj {
    fn into(self) -> MetadataRecord {
        MetadataRecord::Unknown(self)
    }
}
