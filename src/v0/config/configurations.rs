use crate::{
    common::traits::private::Sealed,
    v0::{raw::VariableLengthEnum, tokens::ConfigToken},
};

use super::{error::EnumConfigurationError, traits::EnumConfiguration};
/// The [`Integer Format`](https://github.com/jiricekcz/fef-specification/blob/main/configuration/Integer%20Format.md) configuration option of FEF.
///
/// This configuration option determines how integers are read and written.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntFormat {
    /// 8-bit signed integer
    I8 = 0x00,
    /// 16-bit signed integer
    I16 = 0x01,
    /// 32-bit signed integer
    I32 = 0x02,
    /// 64-bit signed integer
    I64 = 0x03,
    /// 8-bit unsigned integer
    U8 = 0x10,
    /// 16-bit unsigned integer
    U16 = 0x11,
    /// 32-bit unsigned integer
    U32 = 0x12,
    /// 64-bit unsigned integer
    U64 = 0x13,
}

/// Default option for `IntFormat` is `I64`.
///
/// # Examples
/// ```rust
/// # use fef::v0::config::IntFormat;
/// assert_eq!(IntFormat::default(), IntFormat::I64);
/// ```
impl Default for IntFormat {
    fn default() -> Self {
        IntFormat::I64
    }
}

impl TryFrom<VariableLengthEnum> for IntFormat {
    type Error = EnumConfigurationError;

    /// Converts an identifier ([`VariableLengthEnum`]) to an `IntFormat`.
    ///
    /// Fails if the identifier is not recognized as a valid `IntFormat`.
    ///
    /// # Examples
    ///
    /// Converting valid identifiers:
    /// ```rust
    /// # use fef::v0::config::IntFormat;
    /// # use std::convert::TryFrom;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// assert_eq!(IntFormat::try_from(VariableLengthEnum::from(0x00)), Ok(IntFormat::I8));
    /// assert_eq!(IntFormat::try_from(VariableLengthEnum::from(0x01)), Ok(IntFormat::I16));
    /// assert_eq!(IntFormat::try_from(VariableLengthEnum::from(0x02)), Ok(IntFormat::I32));
    /// assert_eq!(IntFormat::try_from(VariableLengthEnum::from(0x03)), Ok(IntFormat::I64));
    /// assert_eq!(IntFormat::try_from(VariableLengthEnum::from(0x10)), Ok(IntFormat::U8));
    /// assert_eq!(IntFormat::try_from(VariableLengthEnum::from(0x11)), Ok(IntFormat::U16));
    /// assert_eq!(IntFormat::try_from(VariableLengthEnum::from(0x12)), Ok(IntFormat::U32));
    /// assert_eq!(IntFormat::try_from(VariableLengthEnum::from(0x13)), Ok(IntFormat::U64));
    /// ```
    ///
    /// Failing to convert an invalid identifier:
    /// ```rust
    /// # use fef::v0::config::IntFormat;
    /// # use std::convert::TryFrom;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// assert!(IntFormat::try_from(VariableLengthEnum::from(0x05)).is_err());
    /// ```
    ///
    /// Reading from a byte stream:
    /// ```rust
    /// # use fef::v0::config::IntFormat;
    /// # use std::convert::TryFrom;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::ReadFrom;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bytes: Vec<u8> = vec![0x03];
    /// let mut reader = &mut bytes.as_slice();
    /// let value = VariableLengthEnum::read_from(reader, &DEFAULT_CONFIG)?;
    /// let int_format = IntFormat::try_from(value)?;
    /// assert_eq!(int_format, IntFormat::I64);
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: VariableLengthEnum) -> Result<Self, Self::Error> {
        let value2 = value.clone();
        let as_usize: usize =
            value
                .clone()
                .try_into()
                .map_err(|_| EnumConfigurationError::IdentifierTooLarge {
                    identifier: value,
                    configuration: Self::token(),
                })?;
        match as_usize {
            0x00 => Ok(IntFormat::I8),
            0x01 => Ok(IntFormat::I16),
            0x02 => Ok(IntFormat::I32),
            0x03 => Ok(IntFormat::I64),
            0x10 => Ok(IntFormat::U8),
            0x11 => Ok(IntFormat::U16),
            0x12 => Ok(IntFormat::U32),
            0x13 => Ok(IntFormat::U64),
            _ => Err(EnumConfigurationError::IdentifierNotRecognized {
                identifier: value2,
                configuration: Self::token(),
            }),
        }
    }
}

impl Sealed for IntFormat {}
impl EnumConfiguration for IntFormat {
    fn value(&self) -> usize {
        *self as usize
    }

    fn token() -> ConfigToken {
        ConfigToken::IntFormat
    }
}

/// The [`Float Format`](https://github.com/jiricekcz/fef-specification/blob/main/configuration/Float%20Format.md) configuration option of FEF.
///
/// This configuration option determines how floats are read and written.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFormat {
    /// 32-bit binary floating point number
    F32 = 0x01,
    /// 64-bit binary floating point number
    F64 = 0x02,
}

/// Default option for `FloatFormat` is `F64`.
///
/// # Examples
/// ```rust
/// # use fef::v0::config::FloatFormat;
/// assert_eq!(FloatFormat::default(), FloatFormat::F64);
/// ```
impl Default for FloatFormat {
    fn default() -> Self {
        FloatFormat::F64
    }
}

impl TryFrom<VariableLengthEnum> for FloatFormat {
    type Error = EnumConfigurationError;

    /// Converts an identifier ([`VariableLengthEnum`]) to a `FloatFormat`.
    ///
    /// Fails if the identifier is not recognized as a valid `FloatFormat`.
    ///
    /// # Examples
    ///
    /// Converting valid identifiers:
    /// ```rust
    /// # use fef::v0::config::FloatFormat;
    /// # use std::convert::TryFrom;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// assert_eq!(FloatFormat::try_from(VariableLengthEnum::from(0x01)), Ok(FloatFormat::F32));
    /// assert_eq!(FloatFormat::try_from(VariableLengthEnum::from(0x02)), Ok(FloatFormat::F64));
    /// ```
    ///
    /// Failing to convert an invalid identifier:
    /// ```rust
    /// # use fef::v0::config::FloatFormat;
    /// # use std::convert::TryFrom;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// assert!(FloatFormat::try_from(VariableLengthEnum::from(0x03)).is_err());
    /// ```
    ///
    /// Reading from a byte stream:
    /// ```rust
    /// # use fef::v0::config::FloatFormat;
    /// # use std::convert::TryFrom;
    /// # use fef::v0::raw::VariableLengthEnum;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::ReadFrom;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bytes: Vec<u8> = vec![0x02];
    /// let mut reader = &mut bytes.as_slice();
    /// let value = VariableLengthEnum::read_from(reader, &DEFAULT_CONFIG)?;
    /// let float_format = FloatFormat::try_from(value)?;
    /// assert_eq!(float_format, FloatFormat::F64);
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: VariableLengthEnum) -> Result<Self, Self::Error> {
        let value2 = value.clone();
        let as_usize: usize =
            value
                .clone()
                .try_into()
                .map_err(|_| EnumConfigurationError::IdentifierTooLarge {
                    identifier: value,
                    configuration: Self::token(),
                })?;
        match as_usize {
            0x01 => Ok(FloatFormat::F32),
            0x02 => Ok(FloatFormat::F64),
            _ => Err(EnumConfigurationError::IdentifierNotRecognized {
                identifier: value2,
                configuration: Self::token(),
            }),
        }
    }
}

impl Sealed for FloatFormat {}

impl EnumConfiguration for FloatFormat {
    fn value(&self) -> usize {
        *self as usize
    }
    fn token() -> ConfigToken {
        ConfigToken::FloatFormat
    }
}
