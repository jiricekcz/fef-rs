use std::io::Read;

use crate::v0::config;
use crate::v0::traits::{private::Sealed, ReadFrom};

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
    R: Read + ?Sized,
{
    type ReadError = FloatReadError;

    /// Reads a float from the given byte stream according to the given configuration.
    ///
    /// Reads a float in the big endian format (according to the FEF specification).  
    ///
    /// # Example
    /// ```rust
    /// # use std::io::Read;
    /// # use fef::v0::traits::ReadFrom;
    /// # use fef::v0::config::OverridableConfig;
    /// # use fef::v0::raw::Float;
    /// # use std::io::Bytes;
    /// # fn main() -> Result<(), fef::v0::raw::error::FloatReadError> {
    ///
    /// let file = vec![0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18];
    /// let mut file_reader = file.as_slice();
    ///
    /// let configuration = OverridableConfig::default();
    ///
    /// let float = Float::read_from(&mut file_reader, &configuration)?;
    ///
    /// assert_eq!(float, Float::Float64(3.141592653589793));
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn read_from<C: config::Config>(
        bytes: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        match configuration.float_format() {
            config::FloatFormat::F32 => {
                let mut value: [u8; 4] = [0; 4];
                bytes.read_exact(&mut value)?;
                let float = f32::from_be_bytes(value);
                Ok(Float::Float32(float))
            }
            config::FloatFormat::F64 => {
                let mut value: [u8; 8] = [0; 8];
                bytes.read_exact(&mut value)?;
                let float = f64::from_be_bytes(value);
                Ok(Float::Float64(float))
            }
        }
    }
}
