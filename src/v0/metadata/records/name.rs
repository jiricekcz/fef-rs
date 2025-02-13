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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        string_length + VariableLengthEnum::min_byte_length_of_usize(string_length)
    }
}

impl<R: ?Sized + Read> ReadFrom<R> for NameMetadataRecordObj {
    type ReadError = MetadataRecordReadError;

    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let full_length: usize =
            VariableLengthEnum::read_from(reader, configuration)?.try_into()?;
        let mut reserved_part = reader.take(full_length as u64);
        let name = String::read_from(&mut reserved_part, configuration)?;
        let mut buf = Vec::new();
        reserved_part.read_to_end(&mut buf)?;
        drop(buf);
        Ok(Self::new(name))
    }
}

impl<W: ?Sized + Write> WriteTo<W> for NameMetadataRecordObj {
    type WriteError = MetadataRecordWriteError;
    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        let byte_length_enum = VariableLengthEnum::from(self.byte_length());
        byte_length_enum.write_to(writer, configuration)?;
        self.name.write_to(writer, configuration)?;
        Ok(())
    }
}
