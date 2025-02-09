use thiserror::Error;

use crate::v0::{
    raw::{error::VariableLengthEnumError, VariableLengthEnum},
    tokens::{error::ConfigTokenError, ConfigToken},
};

#[non_exhaustive]
#[derive(Debug, Error, PartialEq, Eq, Hash, Clone)]
pub enum EnumConfigurationError {
    #[error(
        "identifier {identifier} failed a range check for possible {configuration} identifiers"
    )]
    IdentifierTooLarge {
        configuration: ConfigToken,
        identifier: VariableLengthEnum,
    },

    #[error("identifier {identifier} not recognized as a valid {configuration} identifier")]
    IdentifierNotRecognized {
        configuration: ConfigToken,
        identifier: VariableLengthEnum,
    },
}

#[non_exhaustive]
#[derive(Debug, Error)]

pub enum ConfigurationReadError {
    #[error("failed to read configuration from input")]
    IOError(#[from] std::io::Error),
    #[error("failed to read configuration from input")]
    VariableLengthEnumError(#[from] VariableLengthEnumError),
    #[error("failed to identify token from given identifier")]
    ConfigTokenError(#[from] ConfigTokenError),
    #[error("failed to identify configuration from given identifier")]
    EnumConfigurationError(#[from] EnumConfigurationError),
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConfigurationWriteError {
    #[error("failed to write configuration to output")]
    IOError(#[from] std::io::Error),
    #[error("failed to write configuration to output")]
    VariableLengthEnumError(#[from] VariableLengthEnumError),
}
