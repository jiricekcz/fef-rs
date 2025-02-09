use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        metadata::{
            error::{MetadataReadError, MetadataWriteError},
            traits::MetadataRecordObj,
        },
        raw::VariableLengthEnum,
        tokens::{error::MetadataTokenError, MetadataToken},
        traits::{ReadFrom, WriteTo},
    },
};

pub struct UnknownMetadataRecordObj {
    identifier: VariableLengthEnum,
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
    ) -> Result<Self, MetadataReadError> {
        let length: usize = VariableLengthEnum::read_from(reader, configuration)?.try_into()?;
        let mut data = Vec::with_capacity(length);
        reader.take(length as u64).read_to_end(&mut data)?;
        Ok(Self { identifier, data })
    }
}

impl<W: ?Sized + Write> WriteTo<W> for UnknownMetadataRecordObj {
    type WriteError = MetadataWriteError;
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
