use std::convert::Infallible;

use thiserror::Error;

use crate::v0::{
    config::error::ConfigurationReadError, expr::error::ExprReadWithComposerError,
    metadata::error::MetadataReadError,
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
