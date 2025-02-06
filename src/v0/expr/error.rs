use std::convert::Infallible;

use thiserror::Error;

use crate::v0::{
    raw::error::{FloatReadError, IntegerReadError},
    tokens::{error::ExprTokenReadError, ExprToken},
};

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
    ExprTokenReadError(#[from] ExprTokenReadError),
    IntegersReadError(#[from] IntegerReadError),
    FloatsReadError(#[from] FloatReadError),
    VariableLengthEnumError(#[from] crate::v0::raw::error::VariableLengthEnumError),
}

impl From<Infallible> for ExprReadError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Debug, Error)]
#[error("Failed to read expression.")]
pub enum ExprReadWithComposerError<E>
where
    E: std::error::Error,
{
    ReadError(#[from] ExprReadError),
    ComposeError(#[from] ComposeError<E>),
}

#[derive(Debug, Error)]
#[error("Failed to compose expression.")]
#[non_exhaustive]
pub enum DefaultComposeError {
    #[error("Compose for this expression is missing implementation.")]
    ComposeNotImplemented,
}

#[derive(Debug, Error)]
#[error("Failed to compose expression.")]
pub enum ComposeError<E>
where
    E: std::error::Error,
{
    DefaultError(#[from] DefaultComposeError),
    CustomError(E),
}
