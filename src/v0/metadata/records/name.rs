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

/// Formula name [metadata record](https://github.com/jiricekcz/fef-specification/blob/main/metadata/keys/Name.md).
///
/// This metadata record contains the name of a formula.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameMetadataRecordObj {
    name: String,
}

impl NameMetadataRecordObj {
    /// Creates a new name metadata record.
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Returns the name of the formula.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Converts the name metadata record into a string.
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

    /// Reads a name metadata record from a reader.
    ///
    /// # Example
    /// ```rust
    /// # use fef::v0::metadata::NameMetadataRecordObj;
    /// # use fef::v0::traits::ReadFrom;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let data: Vec<u8> = vec![
    ///     0x0C, // Length of the record
    ///     0x0B, // Length of the string
    ///     b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd', // Name
    /// ];
    /// let mut reader = &mut data.as_slice();
    /// let record = NameMetadataRecordObj::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// assert_eq!(record.name(), "Hello World");
    /// # Ok(())
    /// # }
    /// ```
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

    /// Writes the name metadata record to a writer.
    ///
    /// # Example
    /// ```rust
    /// # use fef::v0::metadata::NameMetadataRecordObj;
    /// # use fef::v0::traits::WriteTo;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut file: Vec<u8> = Vec::new();
    /// let record = NameMetadataRecordObj::new("Hello World".to_string());
    ///
    /// record.write_to(&mut file, &DEFAULT_CONFIG)?;
    ///
    /// let expected_result: Vec<u8> = vec![
    ///     0x0C, // Length of the record
    ///     0x0B, // Length of the string
    ///     b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd', // Name
    /// ];
    /// assert_eq!(file, expected_result);
    /// # Ok(())
    /// # }
    /// ```
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

impl Into<MetadataRecord> for NameMetadataRecordObj {
    fn into(self) -> MetadataRecord {
        MetadataRecord::Name(self)
    }
}
