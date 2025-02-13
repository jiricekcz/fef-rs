use std::io::{Read, Write};

use crate::common::traits::private::Sealed;
use crate::v0::config;
use crate::v0::traits::{ReadFrom, WriteTo};

use super::error::{FloatReadError, FloatWriteError};

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
    fn read_from<C: ?Sized + config::Config>(
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

impl<W> WriteTo<W> for Float
where
    W: Write + ?Sized,
{
    type WriteError = FloatWriteError;

    /// Writes a float to the given byte stream according to the given configuration.
    fn write_to<C: ?Sized + config::Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        match configuration.float_format() {
            config::FloatFormat::F32 => {
                let value = match self {
                    Float::Float32(value) => *value,
                    Float::Float64(value) => *value as f32,
                };
                writer.write_all(&value.to_be_bytes())?;
            }
            config::FloatFormat::F64 => {
                let value = match self {
                    Float::Float32(value) => *value as f64,
                    Float::Float64(value) => *value,
                };
                writer.write_all(&value.to_be_bytes())?;
            }
        };
        Ok(())
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float::Float64(value)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float::Float32(value)
    }
}

impl Into<f64> for Float {
    fn into(self) -> f64 {
        match self {
            Float::Float32(value) => value as f64,
            Float::Float64(value) => value,
        }
    }
}

impl Into<f32> for Float {
    fn into(self) -> f32 {
        match self {
            Float::Float32(value) => value,
            Float::Float64(value) => value as f32,
        }
    }
}
