use std::convert::Infallible;

use thiserror::Error;

use crate::v0::tokens::ExprToken;

#[derive(Debug, Error)]
#[error("Expected {expected}, but found {found}.")]
pub struct NonMatchingExprError {
    pub expected: ExprToken,
    pub found: ExprToken,
}

#[derive(Debug, Error)]
#[error("Failed to read expression.")]
#[non_exhaustive]
pub enum ExprReadError {
    IOError(#[from] std::io::Error),
    ExprTokenReadError(#[from] crate::v0::tokens::error::ExprTokenReadError),
    IntegersReadError(#[from] crate::v0::raw::error::IntegerReadError),
    FloatsReadError(#[from] crate::v0::raw::error::FloatReadError),
}

impl From<Infallible> for ExprReadError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
