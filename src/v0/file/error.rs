use std::convert::Infallible;

use thiserror::Error;

use crate::v0::{
    config::error::ConfigurationReadError, expr::error::ExprReadWithComposerError,
    metadata::error::MetadataReadError, tokens::error::FileContentTypeTokenError,
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
pub enum FileReadError {
    #[error("failed to read file content type token")]
    TokenError(#[from] FileContentTypeTokenError),
    #[error("failed to read single formula file")]
    SingleFormulaError(#[from] SingleFormulaReadError),
    #[error("failed to read raw formula file")]
    RawFormulaError(#[from] RawFormulaReadError),
}
