use std::io::Read;
use std::ops::RangeInclusive;

use super::error::{IntegerConversionError, IntegerReadError};
use crate::common::traits::private::Sealed;
use crate::v0::config;
use crate::v0::traits::ReadFrom;

/// Any integer type defined in the FEF specification.
#[non_exhaustive]
#[derive(Debug, Hash, Clone, Copy, Eq, Ord)]
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

/// Unknown signed 64-bit integer.
#[derive(Debug, Clone, Copy, Eq, Hash)]
pub(crate) enum US64 {
    I64(i64),
    U64(u64),
}

impl US64 {
    fn as_unsigned_if_possible(self) -> US64 {
        match &self {
            US64::I64(value) => {
                if *value >= 0 {
                    US64::U64(*value as u64)
                } else {
                    self
                }
            }
            US64::U64(_) => self,
        }
    }
}

impl std::cmp::PartialEq for US64 {
    fn eq(&self, other: &Self) -> bool {
        let (signed, unsigned) = match (*self, *other) {
            (US64::I64(a), US64::I64(b)) => return a == b,
            (US64::U64(a), US64::U64(b)) => return a == b,
            (US64::I64(a), US64::U64(b)) => (a, b),
            (US64::U64(a), US64::I64(b)) => (b, a),
        };

        if signed < 0 {
            return false;
        }

        let signed = signed as u64;
        signed == unsigned
    }
}

impl std::cmp::PartialOrd for US64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (signed, unsigned) = match (*self, *other) {
            (US64::I64(a), US64::I64(b)) => return a.partial_cmp(&b),
            (US64::U64(a), US64::U64(b)) => return a.partial_cmp(&b),
            (US64::I64(a), US64::U64(b)) => (a, b),
            (US64::U64(a), US64::I64(b)) => (b, a),
        };

        if signed < 0 {
            return Some(std::cmp::Ordering::Less);
        }

        let signed = signed as u64;
        signed.partial_cmp(&unsigned)
    }
}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::Int8(value) => write!(f, "{}", value),
            Integer::Int16(value) => write!(f, "{}", value),
            Integer::Int32(value) => write!(f, "{}", value),
            Integer::Int64(value) => write!(f, "{}", value),
            Integer::UInt8(value) => write!(f, "{}", value),
            Integer::UInt16(value) => write!(f, "{}", value),
            Integer::UInt32(value) => write!(f, "{}", value),
            Integer::UInt64(value) => write!(f, "{}", value),
        }
    }
}

impl std::fmt::LowerHex for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::Int8(value) => write!(f, "{:x}", value),
            Integer::Int16(value) => write!(f, "{:x}", value),
            Integer::Int32(value) => write!(f, "{:x}", value),
            Integer::Int64(value) => write!(f, "{:x}", value),
            Integer::UInt8(value) => write!(f, "{:x}", value),
            Integer::UInt16(value) => write!(f, "{:x}", value),
            Integer::UInt32(value) => write!(f, "{:x}", value),
            Integer::UInt64(value) => write!(f, "{:x}", value),
        }
    }
}

impl std::fmt::UpperHex for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::Int8(value) => write!(f, "{:X}", value),
            Integer::Int16(value) => write!(f, "{:X}", value),
            Integer::Int32(value) => write!(f, "{:X}", value),
            Integer::Int64(value) => write!(f, "{:X}", value),
            Integer::UInt8(value) => write!(f, "{:X}", value),
            Integer::UInt16(value) => write!(f, "{:X}", value),
            Integer::UInt32(value) => write!(f, "{:X}", value),
            Integer::UInt64(value) => write!(f, "{:X}", value),
        }
    }
}

impl std::fmt::Binary for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::Int8(value) => write!(f, "{:b}", value),
            Integer::Int16(value) => write!(f, "{:b}", value),
            Integer::Int32(value) => write!(f, "{:b}", value),
            Integer::Int64(value) => write!(f, "{:b}", value),
            Integer::UInt8(value) => write!(f, "{:b}", value),
            Integer::UInt16(value) => write!(f, "{:b}", value),
            Integer::UInt32(value) => write!(f, "{:b}", value),
            Integer::UInt64(value) => write!(f, "{:b}", value),
        }
    }
}

impl std::fmt::Octal for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::Int8(value) => write!(f, "{:o}", value),
            Integer::Int16(value) => write!(f, "{:o}", value),
            Integer::Int32(value) => write!(f, "{:o}", value),
            Integer::Int64(value) => write!(f, "{:o}", value),
            Integer::UInt8(value) => write!(f, "{:o}", value),
            Integer::UInt16(value) => write!(f, "{:o}", value),
            Integer::UInt32(value) => write!(f, "{:o}", value),
            Integer::UInt64(value) => write!(f, "{:o}", value),
        }
    }
}

impl std::fmt::UpperExp for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::Int8(value) => write!(f, "{:E}", value),
            Integer::Int16(value) => write!(f, "{:E}", value),
            Integer::Int32(value) => write!(f, "{:E}", value),
            Integer::Int64(value) => write!(f, "{:E}", value),
            Integer::UInt8(value) => write!(f, "{:E}", value),
            Integer::UInt16(value) => write!(f, "{:E}", value),
            Integer::UInt32(value) => write!(f, "{:E}", value),
            Integer::UInt64(value) => write!(f, "{:E}", value),
        }
    }
}

impl std::fmt::LowerExp for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::Int8(value) => write!(f, "{:e}", value),
            Integer::Int16(value) => write!(f, "{:e}", value),
            Integer::Int32(value) => write!(f, "{:e}", value),
            Integer::Int64(value) => write!(f, "{:e}", value),
            Integer::UInt8(value) => write!(f, "{:e}", value),
            Integer::UInt16(value) => write!(f, "{:e}", value),
            Integer::UInt32(value) => write!(f, "{:e}", value),
            Integer::UInt64(value) => write!(f, "{:e}", value),
        }
    }
}

impl From<i8> for Integer {
    fn from(value: i8) -> Self {
        Integer::Int8(value)
    }
}

impl From<i16> for Integer {
    fn from(value: i16) -> Self {
        Integer::Int16(value)
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Integer::Int32(value)
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Integer::Int64(value)
    }
}

impl From<u8> for Integer {
    fn from(value: u8) -> Self {
        Integer::UInt8(value)
    }
}

impl From<u16> for Integer {
    fn from(value: u16) -> Self {
        Integer::UInt16(value)
    }
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer::UInt32(value)
    }
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Integer::UInt64(value)
    }
}

impl From<Integer> for US64 {
    fn from(value: Integer) -> Self {
        match value {
            Integer::Int8(value) => US64::I64(value as i64),
            Integer::Int16(value) => US64::I64(value as i64),
            Integer::Int32(value) => US64::I64(value as i64),
            Integer::Int64(value) => US64::I64(value),
            Integer::UInt8(value) => US64::U64(value as u64),
            Integer::UInt16(value) => US64::U64(value as u64),
            Integer::UInt32(value) => US64::U64(value as u64),
            Integer::UInt64(value) => US64::U64(value),
        }
    }
}

impl std::cmp::PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        let a = US64::from(*self);
        let b = US64::from(*other);

        a.eq(&b)
    }
}

impl std::cmp::PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = US64::from(*self);
        let b = US64::from(*other);

        a.partial_cmp(&b)
    }
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
    fn read_from<C: ?Sized + config::Config>(
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

impl TryInto<i64> for Integer {
    type Error = IntegerConversionError;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Integer::Int8(value) => Ok(value as i64),
            Integer::Int16(value) => Ok(value as i64),
            Integer::Int32(value) => Ok(value as i64),
            Integer::Int64(value) => Ok(value),
            Integer::UInt8(value) => Ok(value as i64),
            Integer::UInt16(value) => Ok(value as i64),
            Integer::UInt32(value) => Ok(value as i64),
            Integer::UInt64(value) => {
                if value <= i64::MAX as u64 {
                    Ok(value as i64)
                } else {
                    Err(IntegerConversionError::OutOfRange {
                        value: value.into(),
                        range: RangeInclusive::new(i64::MIN.into(), i64::MAX.into()),
                    })
                }
            }
        }
    }
}

impl TryInto<u64> for Integer {
    type Error = IntegerConversionError;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Integer::Int8(value) => {
                if value >= 0 {
                    Ok(value as u64)
                } else {
                    Err(IntegerConversionError::OutOfRange {
                        value: value.into(),
                        range: RangeInclusive::new(0.into(), u64::MAX.into()),
                    })
                }
            }
            Integer::Int16(value) => {
                if value >= 0 {
                    Ok(value as u64)
                } else {
                    Err(IntegerConversionError::OutOfRange {
                        value: value.into(),
                        range: RangeInclusive::new(0.into(), u64::MAX.into()),
                    })
                }
            }
            Integer::Int32(value) => {
                if value >= 0 {
                    Ok(value as u64)
                } else {
                    Err(IntegerConversionError::OutOfRange {
                        value: value.into(),
                        range: RangeInclusive::new(0.into(), u64::MAX.into()),
                    })
                }
            }
            Integer::Int64(value) => {
                if value >= 0 {
                    Ok(value as u64)
                } else {
                    Err(IntegerConversionError::OutOfRange {
                        value: value.into(),
                        range: RangeInclusive::new(0.into(), u64::MAX.into()),
                    })
                }
            }
            Integer::UInt8(value) => Ok(value as u64),
            Integer::UInt16(value) => Ok(value as u64),
            Integer::UInt32(value) => Ok(value as u64),
            Integer::UInt64(value) => Ok(value),
        }
    }
}

impl Integer {
    /// Creates a new [Integer] with using the smallest possible integer type. If possible, will choose a signed integer.
    pub fn compact(&self) -> Integer {
        let value = US64::from(*self);

        match value.as_unsigned_if_possible() {
            US64::I64(value) => {
                // Binary search for the smallest integer type that can hold the value.
                if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
                    if value >= i8::MIN as i64 && value <= i8::MAX as i64 {
                        Integer::Int8(value as i8)
                    } else {
                        Integer::Int16(value as i16)
                    }
                } else {
                    if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                        Integer::Int32(value as i32)
                    } else {
                        Integer::Int64(value)
                    }
                }
            }
            US64::U64(value) => {
                // Binary search for the smallest integer type that can hold the value.
                if value <= u16::MAX as u64 {
                    if value <= u8::MAX as u64 {
                        if value <= i8::MAX as u64 {
                            Integer::Int8(value as i8)
                        } else {
                            Integer::UInt8(value as u8)
                        }
                    } else {
                        if value <= i16::MAX as u64 {
                            Integer::Int16(value as i16)
                        } else {
                            Integer::UInt16(value as u16)
                        }
                    }
                } else {
                    if value <= u32::MAX as u64 {
                        if value <= i32::MAX as u64 {
                            Integer::Int32(value as i32)
                        } else {
                            Integer::UInt32(value as u32)
                        }
                    } else {
                        if value <= i64::MAX as u64 {
                            Integer::Int64(value as i64)
                        } else {
                            Integer::UInt64(value)
                        }
                    }
                }
            }
        }
    }
    /// Creates a new [Integer] with using the smallest possible integer type. If the value is non-negative, will choose an unsigned integer.
    pub fn compact_unsigned(&self) -> Integer {
        let value = US64::from(*self);

        match value.as_unsigned_if_possible() {
            US64::I64(value) => {
                // Binary search for the smallest integer type that can hold the value.
                if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
                    if value >= i8::MIN as i64 && value <= i8::MAX as i64 {
                        Integer::Int8(value as i8)
                    } else {
                        Integer::Int16(value as i16)
                    }
                } else {
                    if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                        Integer::Int32(value as i32)
                    } else {
                        Integer::Int64(value)
                    }
                }
            }

            US64::U64(value) => {
                // Binary search for the smallest integer type that can hold the value.
                if value <= u16::MAX as u64 {
                    if value <= u8::MAX as u64 {
                        Integer::UInt8(value as u8)
                    } else {
                        Integer::UInt16(value as u16)
                    }
                } else {
                    if value <= u32::MAX as u64 {
                        Integer::UInt32(value as u32)
                    } else {
                        Integer::UInt64(value)
                    }
                }
            }
        }
    }
}
