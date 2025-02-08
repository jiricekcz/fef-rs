use thiserror::Error;

use crate::v0::{raw::VariableLengthEnum, tokens::ConfigToken};

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
