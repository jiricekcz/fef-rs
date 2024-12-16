use std::io::Read;

use crate::traits::{private::Sealed, ReadFrom};

use super::error::IntegerReadError;

/// Any integer type defined in the FEF specification.
#[non_exhaustive]
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

    fn read_from_bytes<C: crate::config::Config>(
        reader: &mut std::io::Bytes<R>,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        todo!("Implement parsing integer from bytes.")
    }
}
