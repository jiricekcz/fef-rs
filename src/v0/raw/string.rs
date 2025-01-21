use crate::traits::{private::Sealed, ReadFrom};
use std::io::Read;
use std::string::String;

use super::error::StringReadError;

impl Sealed for String {}

impl<R> ReadFrom<R> for String
where
    R: Read,
{
    type ReadError = StringReadError;

    /// Reads a string from the given byte stream according to the given configuration.
    ///
    /// # Example
    /// ```rust
    /// # use std::io::Read;
    /// # use fef::traits::ReadFrom;
    /// # use fef::config::OverridableConfig;
    /// # use std::string::String;
    /// # use std::io::Bytes;
    /// # fn main() -> Result<(), fef::raw::error::StringReadError> {
    /// let file = vec![0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F];
    /// let mut bytes = file.bytes();
    ///
    /// let configuration = OverridableConfig::default();
    ///
    /// let string = String::read_from_bytes(&mut bytes, &configuration)?;
    /// assert_eq!(string, "Hello");
    ///
    /// # Ok(())
    /// # }
    ///```
    fn read_from_bytes<C: crate::config::Config>(
        bytes: &mut std::io::Bytes<R>,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let length: usize =
            crate::raw::VariableLengthEnum::read_from_bytes(&mut *bytes, &*configuration)?
                .try_into()
                .map_err(|_| StringReadError::LengthTooLarge)?;

        let buffer = crate::raw::bytes::read_dynamic(bytes, length)?;

        let parsed_utf8: String = String::from_utf8(buffer)?;

        Ok(parsed_utf8)
    }
}
