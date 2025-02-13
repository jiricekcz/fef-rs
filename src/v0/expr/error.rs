use std::convert::Infallible;

use thiserror::Error;

use crate::v0::{
    raw::error::{
        FloatReadError, FloatWriteError, IntegerReadError, IntegerWriteError,
        VariableLengthEnumError,
    },
    tokens::{
        error::{ExprTokenReadError, ExprTokenWriteError},
        ExprToken,
    },
};

#[derive(Debug, Error)]
#[error("expected {expected}, but found {found}.")]
pub struct NonMatchingExprError {
    pub expected: ExprToken,
    pub found: ExprToken,
}

#[derive(Debug, Error)]
#[error("failed to read expression.")]
#[non_exhaustive]
pub enum ExprReadError {
    IOError(#[from] std::io::Error),
    ExprTokenReadError(#[from] ExprTokenReadError),
    IntegersReadError(#[from] IntegerReadError),
    FloatsReadError(#[from] FloatReadError),
    VariableLengthEnumError(#[from] VariableLengthEnumError),
}

impl From<Infallible> for ExprReadError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Debug, Error)]
#[error("failed to read expression.")]
pub enum ExprReadWithComposerError<E>
where
    E: std::error::Error,
{
    ReadError(#[from] ExprReadError),
    ComposeError(#[from] ComposeError<E>),
}

#[derive(Debug, Error)]
#[error("failed to compose expression.")]
#[non_exhaustive]
pub enum DefaultComposeError {
    #[error("compose for this expression is missing implementation.")]
    ComposeNotImplemented,
}

#[derive(Debug, Error)]
#[error("failed to compose expression.")]
pub enum ComposeError<E>
where
    E: std::error::Error,
{
    DefaultError(#[from] DefaultComposeError),
    CustomError(E),
}

#[derive(Debug, Error)]
#[error("failed to write expression.")]
pub enum ExprWriteError {
    VariableLengthEnumError(#[from] VariableLengthEnumError),
    IntegersWriteError(#[from] IntegerWriteError),
    FloatsWriteError(#[from] FloatWriteError),
    ExprTokenWriteError(#[from] ExprTokenWriteError),
}

#[derive(Debug, Error)]
#[error("failed to decompose expression.")]
pub enum DefaultDecomposeError {
    DecomposeNotImplemented,
}

#[derive(Debug, Error)]
#[error("failed to decompose expression.")]
pub enum DecomposeError<E>
where
    E: std::error::Error,
{
    DefaultError(#[from] DefaultDecomposeError),
    CustomError(E),
}

#[derive(Debug, Error)]
#[error("failed to read expression.")]
pub enum ExprWriteWithDecomposerError<E>
where
    E: std::error::Error,
{
    WriteError(#[from] ExprWriteError),
    DecomposeError(#[from] DecomposeError<E>),
}
