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
/// Formula variable name [metadata record](https://github.com/jiricekcz/fef-specification/blob/main/metadata/keys/Variable%20Name.md).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableNameMetadataRecordObj {
    name: String,
    variable_identifier: VariableLengthEnum,
}

impl VariableNameMetadataRecordObj {
    /// Creates a new variable name metadata record.
    ///
    /// # Example
    ///
    /// Creating a metadata record setting the name of variable number `1` to `"x"`:
    /// ```rust
    /// # use fef::v0::metadata::VariableNameMetadataRecordObj;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let variable_identifier = VariableLengthEnum::from(1);
    /// let record = VariableNameMetadataRecordObj::new("x".to_string(), variable_identifier.clone());
    ///
    /// assert_eq!(record.name(), "x");
    /// assert_eq!(record.variable_identifier(), &variable_identifier);
    /// # Ok(())
    /// # }
    pub fn new(name: String, variable_identifier: VariableLengthEnum) -> Self {
        Self {
            name,
            variable_identifier,
        }
    }

    /// Returns the name of the variable to which this metadata record refers.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the identifier of the variable to which this metadata record refers.
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
    type ReadError = MetadataRecordReadError;

    /// Reads a variable name metadata record from a reader.
    ///
    /// # Example
    /// ```rust
    /// # use fef::v0::metadata::VariableNameMetadataRecordObj;
    /// # use fef::v0::traits::ReadFrom;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let data: Vec<u8> = vec![
    ///    0x03, // Length of the record
    ///    0x01, // Variable identifier
    ///    0x01, // Length of the string
    ///    b'x', // Name
    /// ];
    /// let mut reader = &mut data.as_slice();
    /// let record = VariableNameMetadataRecordObj::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// assert_eq!(record.name(), "x");
    /// assert_eq!(record.variable_identifier(), &VariableLengthEnum::from(1));
    /// # Ok(())
    /// # }
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
    type WriteError = MetadataRecordWriteError;

    /// Writes the variable name metadata record to a writer.
    ///
    /// # Example
    ///
    /// Writing a metadata record setting the name of variable number `1` to `"x"`:
    /// ```rust
    /// # use fef::v0::metadata::VariableNameMetadataRecordObj;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::WriteTo;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let variable_identifier = VariableLengthEnum::from(1);
    /// let record = VariableNameMetadataRecordObj::new("x".to_string(), variable_identifier.clone());
    ///
    /// let mut writer: Vec<u8> = Vec::new();
    /// record.write_to(&mut writer, &DEFAULT_CONFIG)?;
    ///
    /// let expected_result: Vec<u8> = vec![
    ///    0x03, // Length of the record
    ///    0x01, // Variable identifier
    ///    0x01, // Length of the string
    ///    b'x', // Name
    /// ];
    /// assert_eq!(writer, expected_result);
    /// # Ok(())
    /// # }
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
