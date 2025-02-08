use crate::v0::raw::VariableLengthEnum;

use super::error::ConfigTokenError;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
#[non_exhaustive]
pub enum ConfigToken {
    FloatFormat = 0x00,
    IntFormat = 0x01,
}

impl ConfigToken {
    pub fn is_enum_configuration(&self) -> bool {
        *self as usize <= 0x7F
    }
    pub fn variable_length_enum(&self) -> VariableLengthEnum {
        VariableLengthEnum::from(*self as usize)
    }
}

impl TryFrom<VariableLengthEnum> for ConfigToken {
    type Error = ConfigTokenError;
    fn try_from(value: VariableLengthEnum) -> Result<Self, Self::Error> {
        let value2 = value.clone();
        let int_value: usize = value
            .clone()
            .try_into()
            .map_err(move |_| ConfigTokenError::IdentifierTooLarge { identifier: value2 })?;
        match int_value {
            0x00 => Ok(ConfigToken::FloatFormat),
            0x01 => Ok(ConfigToken::IntFormat),
            _ => Err(ConfigTokenError::IdentifierNotRecognized { identifier: value }),
        }
    }
}

impl std::fmt::Display for ConfigToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigToken::FloatFormat => write!(f, "Float Format"),
            ConfigToken::IntFormat => write!(f, "Integer Format"),
        }
    }
}
