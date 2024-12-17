use std::io::Read;

use crate::traits::{private::Sealed, ReadFrom};

use super::error::IntegerReadError;

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
    R: Read,
{
    type ReadError = IntegerReadError;

    /// Reads an integer from the given byte stream according to the given configuration.
    ///
    /// Reads an integer in the big endian format (according to the FEF specification).  
    ///
    /// # Example
    /// ```rust
    /// # use std::io::Read;
    /// # use fef::traits::ReadFrom;
    /// # use fef::config::OverridableConfig;
    /// # use fef::raw::Integer;
    /// # use std::io::Bytes;
    /// # fn main() -> Result<(), fef::raw::error::IntegerReadError> {
    ///
    /// let file = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    /// let mut bytes = file.bytes();
    ///
    /// let configuration = OverridableConfig::default();
    ///
    /// let uint64 = Integer::read_from_bytes(&mut bytes, &configuration)?;
    /// assert_eq!(uint64, Integer::Int64(0x0102030405060708));
    ///
    /// # Ok(())
    /// # }
    ///```
    fn read_from_bytes<C: crate::config::Config>(
        bytes: &mut std::io::Bytes<R>,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        match configuration.integer_format() {
            crate::config::IntFormat::I8 => {
                let value = bytes.next().ok_or(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "unexpected end of stream",
                ))??;
                Ok(Integer::Int8(i8::from_be_bytes([value])))
            }
            crate::config::IntFormat::I16 => {
                let value = crate::raw::bytes::read_exact::<2, R>(bytes)?;
                Ok(Integer::Int16(i16::from_be_bytes(value)))
            }
            crate::config::IntFormat::I32 => {
                let value = crate::raw::bytes::read_exact::<4, R>(bytes)?;
                Ok(Integer::Int32(i32::from_be_bytes(value)))
            }
            crate::config::IntFormat::I64 => {
                let value = crate::raw::bytes::read_exact::<8, R>(bytes)?;
                Ok(Integer::Int64(i64::from_be_bytes(value)))
            }
            crate::config::IntFormat::U8 => {
                let value = bytes.next().ok_or(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "unexpected end of stream",
                ))??;
                Ok(Integer::UInt8(value))
            }
            crate::config::IntFormat::U16 => {
                let value = crate::raw::bytes::read_exact::<2, R>(bytes)?;
                Ok(Integer::UInt16(u16::from_be_bytes(value)))
            }
            crate::config::IntFormat::U32 => {
                let value = crate::raw::bytes::read_exact::<4, R>(bytes)?;
                Ok(Integer::UInt32(u32::from_be_bytes(value)))
            }
            crate::config::IntFormat::U64 => {
                let value = crate::raw::bytes::read_exact::<8, R>(bytes)?;
                Ok(Integer::UInt64(u64::from_be_bytes(value)))
            }
        }
    }
}
