use std::io::Read;

use super::error::IntegerReadError;
use crate::v0::config;
use crate::v0::traits::{private::Sealed, ReadFrom};

/// Any integer type defined in the FEF specification.
#[non_exhaustive]
#[derive(Debug, Hash, Clone, Copy, PartialEq)]
pub enum Integer {
    /// 8-bit signed integer.
    Int8(i8),

    /// 16-bit signed integer.
    Int16(i16),

    /// 32-bit signed integer.
    Int32(i32),

    /// 64-bit signed integer.
    Int64(i64),

    /// 8-bit unsigned integer.
    UInt8(u8),

    /// 16-bit unsigned integer.
    UInt16(u16),

    /// 32-bit unsigned integer.
    UInt32(u32),

    /// 64-bit unsigned integer.
    UInt64(u64),
}

impl Sealed for Integer {}

impl<R> ReadFrom<R> for Integer
where
    R: Read + ?Sized,
{
    type ReadError = IntegerReadError;

    /// Reads an integer from the given byte stream according to the given configuration.
    ///
    /// Reads an integer in the big endian format (according to the FEF specification).  
    ///
    /// # Example
    /// ```rust
    /// # use std::io::Read;
    /// # use fef::v0::traits::ReadFrom;
    /// # use fef::v0::config::OverridableConfig;
    /// # use fef::v0::raw::Integer;
    /// # use std::io::Bytes;
    /// # fn main() -> Result<(), fef::v0::raw::error::IntegerReadError> {
    ///
    /// let file = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    /// let mut file_reader = file.as_slice();
    ///
    /// let configuration = OverridableConfig::default();
    ///
    /// let uint64 = Integer::read_from(&mut file_reader, &configuration)?;
    /// assert_eq!(uint64, Integer::Int64(0x0102030405060708));
    ///
    /// # Ok(())
    /// # }
    ///```
    fn read_from<C: config::Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        match configuration.integer_format() {
            config::IntFormat::I8 => {
                let mut value: [u8; 1] = [0; 1];
                reader.read_exact(&mut value)?;
                Ok(Integer::Int8(i8::from_be_bytes(value)))
            }
            config::IntFormat::I16 => {
                let mut value: [u8; 2] = [0; 2];
                reader.read_exact(&mut value)?;
                Ok(Integer::Int16(i16::from_be_bytes(value)))
            }
            config::IntFormat::I32 => {
                let mut value: [u8; 4] = [0; 4];
                reader.read_exact(&mut value)?;
                Ok(Integer::Int32(i32::from_be_bytes(value)))
            }
            config::IntFormat::I64 => {
                let mut value: [u8; 8] = [0; 8];
                reader.read_exact(&mut value)?;
                Ok(Integer::Int64(i64::from_be_bytes(value)))
            }
            config::IntFormat::U8 => {
                let mut value: [u8; 1] = [0; 1];
                reader.read_exact(&mut value)?;
                Ok(Integer::UInt8(u8::from_be_bytes(value)))
            }
            config::IntFormat::U16 => {
                let mut value: [u8; 2] = [0; 2];
                reader.read_exact(&mut value)?;
                Ok(Integer::UInt16(u16::from_be_bytes(value)))
            }
            config::IntFormat::U32 => {
                let mut value: [u8; 4] = [0; 4];
                reader.read_exact(&mut value)?;
                Ok(Integer::UInt32(u32::from_be_bytes(value)))
            }
            config::IntFormat::U64 => {
                let mut value: [u8; 8] = [0; 8];
                reader.read_exact(&mut value)?;
                Ok(Integer::UInt64(u64::from_be_bytes(value)))
            }
        }
    }
}
