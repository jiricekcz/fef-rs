use crate::{
    common::traits::private::Sealed,
    v0::{raw::VariableLengthEnum, tokens::ConfigToken},
};

use super::{error::EnumConfigurationError, traits::EnumConfiguration};
/// The `Integer Format` configuration option of FEF.
///
/// This configuration option determines how integers are read and written.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntFormat {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
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
            0 => Ok(IntFormat::I8),
            1 => Ok(IntFormat::I16),
            2 => Ok(IntFormat::I32),
            3 => Ok(IntFormat::I64),
            4 => Ok(IntFormat::U8),
            5 => Ok(IntFormat::U16),
            6 => Ok(IntFormat::U32),
            7 => Ok(IntFormat::U64),
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

/// The `Float Format` configuration option of FEF.
///
/// This configuration option determines how floats are read and written.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFormat {
    F32,
    F64,
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
            0 => Ok(FloatFormat::F32),
            1 => Ok(FloatFormat::F64),
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
