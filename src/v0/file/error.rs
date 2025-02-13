use std::convert::Infallible;

use thiserror::Error;

use crate::v0::{
    config::error::{ConfigurationReadError, ConfigurationWriteError},
    expr::error::{ExprReadWithComposerError, ExprWriteWithDecomposerError},
    metadata::error::{FromIteratorMetadataWriteError, MetadataReadError},
    raw::error::VariableLengthEnumError,
    tokens::error::FileContentTypeTokenError,
};

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SingleFormulaReadError {
    #[error("failed to read a configuration")]
    ConfigurationReadError(#[from] ConfigurationReadError),
    #[error("failed to read metadata")]
    MetadataReadError(#[from] MetadataReadError),
    #[error("failed to read expression")]
    ExprReadError(#[from] ExprReadWithComposerError<Infallible>),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RawFormulaReadError {
    #[error("failed to read expression")]
    ExprReadError(#[from] ExprReadWithComposerError<Infallible>),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RawFormulaWriteError<E: std::error::Error> {
    #[error("failed to write expression")]
    ExprWriteError(#[from] ExprWriteWithDecomposerError<E>),
    #[error("failed to write major version")]
    VersionWriteError(VariableLengthEnumError),
    #[error("failed to write file content type token")]
    TokenError(#[from] FileContentTypeTokenError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SingleFormulaWriteError<E: std::error::Error, EM: std::error::Error> {
    #[error("failed to write expression")]
    ExprWriteError(#[from] ExprWriteWithDecomposerError<E>),
    #[error("failed to write a configuration")]
    ConfigurationWriteError(#[from] ConfigurationWriteError),
    #[error("failed to write metadata")]
    MetadataWriteError(#[from] FromIteratorMetadataWriteError<EM>),
    #[error("failed to write major version")]
    VersionWriteError(VariableLengthEnumError),
    #[error("failed to write file content type token")]
    TokenError(#[from] FileContentTypeTokenError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FileReadError {
    #[error("failed to read file content type token")]
    TokenError(#[from] FileContentTypeTokenError),
    #[error("failed to read single formula file")]
    SingleFormulaError(#[from] SingleFormulaReadError),
    #[error("failed to read raw formula file")]
    RawFormulaError(#[from] RawFormulaReadError),
}
