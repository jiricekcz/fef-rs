use std::io::Read;

use crate::traits::{private::Sealed, ReadFrom};

use super::error::FloatReadError;

/// Any float type defined in the FEF specification.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Float {
    /// 32-bit floating point number.
    Float32(f32),

    /// 64-bit floating point number.
    Float64(f64),
}

impl Sealed for Float {}

impl<R> ReadFrom<R> for Float
where
    R: Read,
{
    type ReadError = FloatReadError;

    /// Reads a float from the given byte stream according to the given configuration.
    ///
    /// Reads a float in the big endian format (according to the FEF specification).  
    ///
    /// # Example
    /// ```rust
    /// # use std::io::Read;
    /// # use fef::traits::ReadFrom;
    /// # use fef::config::OverridableConfig;
    /// # use fef::raw::Float;
    /// # use std::io::Bytes;
    /// # fn main() -> Result<(), fef::raw::error::FloatReadError> {
    ///
    /// let file = vec![0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18];
    /// let mut bytes = file.bytes();
    ///
    /// let configuration = OverridableConfig::default();
    ///
    /// let float = Float::read_from_bytes(&mut bytes, &configuration)?;
    ///
    /// assert_eq!(float, Float::Float64(3.141592653589793));
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn read_from_bytes<C: crate::config::Config>(
        bytes: &mut std::io::Bytes<R>,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        match configuration.float_format() {
            crate::config::FloatFormat::F32 => {
                let value = crate::raw::bytes::read_exact::<4, R>(bytes)?;
                let float = f32::from_be_bytes(value);
                Ok(Float::Float32(float))
            }
            crate::config::FloatFormat::F64 => {
                let value = crate::raw::bytes::read_exact::<8, R>(bytes)?;
                let float = f64::from_be_bytes(value);
                Ok(Float::Float64(float))
            }
        }
    }
}
