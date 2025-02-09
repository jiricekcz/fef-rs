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

pub struct NameMetadataRecordObj {
    name: String,
}

impl NameMetadataRecordObj {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Into<String> for NameMetadataRecordObj {
    fn into(self) -> String {
        self.name
    }
}

impl Sealed for NameMetadataRecordObj {}

impl MetadataRecordObj for NameMetadataRecordObj {
    fn token(&self) -> Result<MetadataToken, MetadataTokenError> {
        Ok(MetadataToken::Name)
    }
    fn byte_length(&self) -> usize {
        let string_length = self.name.len();
        string_length + VariableLengthEnum::min_byte_length_of(string_length)
    }
}

impl<R: ?Sized + Read> ReadFrom<R> for NameMetadataRecordObj {
    type ReadError = MetadataReadError;

    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        _configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let name = String::read_from(reader, _configuration)?;
        Ok(Self::new(name))
    }
}

impl<W: ?Sized + Write> WriteTo<W> for NameMetadataRecordObj {
    type WriteError = MetadataWriteError;
    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        self.name.write_to(writer, configuration)?;
        Ok(())
    }
}
