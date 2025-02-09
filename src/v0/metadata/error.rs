use thiserror::Error;

use crate::v0::raw::error::{StringReadError, StringWriteError};

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataReadError {
    #[error("failed to read a string")]
    StringReadError(#[from] StringReadError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataWriteError {
    #[error("failed to write a string")]
    StringWriteError(#[from] StringWriteError),
}
