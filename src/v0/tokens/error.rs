//! Errors associated with the token module.

use crate::v0 as fef;

use thiserror::Error;

/// Errors that can occur while working with [ExprToken](crate::v0::tokens::ExprToken)s.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ExprTokenError {
    #[error("identifier {identifier} not recognized as a valid expression identifier")]
    IdentifierNotRecognized {
        identifier: fef::raw::VariableLengthEnum,
    },
    #[error("identifier {identifier} failed a range check for possible Expr identifiers")]
    IdentifierTooLarge {
        identifier: fef::raw::VariableLengthEnum,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ExprTokenReadError {
    #[error("failed to read identifier from input")]
    IOError(#[from] std::io::Error),
    #[error("failed to read identifier from input")]
    VariableLengthEnumError(#[from] fef::raw::error::VariableLengthEnumError),
    #[error("failed to identify token from given identifier")]
    ExprTokenError {
        #[from]
        source: ExprTokenError,
    },
}
