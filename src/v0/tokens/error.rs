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
