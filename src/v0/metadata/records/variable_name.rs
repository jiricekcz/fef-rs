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

pub struct VariableNameMetadataRecordObj {
    name: String,
    variable_identifier: VariableLengthEnum,
}

impl VariableNameMetadataRecordObj {
    pub fn new(name: String, variable_identifier: VariableLengthEnum) -> Self {
        Self {
            name,
            variable_identifier,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn variable_identifier(&self) -> &VariableLengthEnum {
        &self.variable_identifier
    }
}

impl Sealed for VariableNameMetadataRecordObj {}

impl MetadataRecordObj for VariableNameMetadataRecordObj {
    fn token(&self) -> Result<MetadataToken, MetadataTokenError> {
        Ok(MetadataToken::VariableName)
    }
    fn byte_length(&self) -> usize {
        let string_length = self.name.len();
        string_length
            + VariableLengthEnum::min_byte_length_of_usize(string_length)
            + self.variable_identifier.min_byte_length()
    }
}

impl<R: ?Sized + Read> ReadFrom<R> for VariableNameMetadataRecordObj {
    type ReadError = MetadataReadError;

    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let full_length: usize =
            VariableLengthEnum::read_from(reader, configuration)?.try_into()?;
        let mut reserved_part = reader.take(full_length as u64);
        let variable_identifier = VariableLengthEnum::read_from(&mut reserved_part, configuration)?;
        let name = String::read_from(&mut reserved_part, configuration)?;
        let mut buf = Vec::new();
        reserved_part.read_to_end(&mut buf)?;
        drop(buf);
        Ok(Self::new(name, variable_identifier))
    }
}

impl<W: ?Sized + Write> WriteTo<W> for VariableNameMetadataRecordObj {
    type WriteError = MetadataWriteError;
    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        let byte_length_enum = VariableLengthEnum::from(self.byte_length());
        byte_length_enum.write_to(writer, configuration)?;
        self.variable_identifier.write_to(writer, configuration)?;
        self.name.write_to(writer, configuration)?;
        Ok(())
    }
}
