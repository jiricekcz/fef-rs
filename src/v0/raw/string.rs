use crate::common::traits::private::Sealed;
use crate::v0::config::Config;
use crate::v0::raw;
use crate::v0::traits::{ReadFrom, WriteTo};
use std::io::{Read, Write};
use std::string::String;

use super::error::{StringReadError, StringWriteError};

impl Sealed for String {}

impl<R> ReadFrom<R> for String
where
    R: Read + ?Sized,
{
    type ReadError = StringReadError;

    /// Reads a string from the given byte stream according to the given configuration.
    ///
    /// # Example
    /// ```rust
    /// # use std::io::Read;
    /// # use fef::v0::traits::ReadFrom;
    /// # use fef::v0::config::OverridableConfig;
    /// # use std::string::String;
    /// # fn main() -> Result<(), fef::v0::raw::error::StringReadError> {
    /// let file = vec![0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F];
    /// let mut file_reader = file.as_slice();
    ///
    /// let configuration = OverridableConfig::default();
    ///
    /// let string = String::read_from(&mut file_reader, &configuration)?;
    /// assert_eq!(string, "Hello");
    ///
    /// # Ok(())
    /// # }
    ///```
    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let length: usize = raw::VariableLengthEnum::read_from(&mut *reader, &*configuration)?
            .try_into()
            .map_err(|_| StringReadError::LengthTooLarge)?;

        let mut buffer: Vec<u8> = Vec::with_capacity(length);
        reader.take(length as u64).read_to_end(&mut buffer)?;

        let parsed_utf8: String = String::from_utf8(buffer)?;

        Ok(parsed_utf8)
    }
}

impl<W> WriteTo<W> for String
where
    W: Write + ?Sized,
{
    type WriteError = StringWriteError;
    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        let bytes = self.as_bytes();
        let length = bytes.len();
        let variable_length_enum = raw::VariableLengthEnum::from(length);
        variable_length_enum.write_to(writer, configuration)?;
        writer.write_all(bytes)?;
        Ok(())
    }
}
