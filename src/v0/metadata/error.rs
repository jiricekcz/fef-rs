use thiserror::Error;

use crate::v0::{
    raw::error::{StringReadError, StringWriteError, VariableLengthEnumError},
    tokens::error::MetadataTokenError,
};

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataReadError {
    #[error("failed to read a string")]
    StringReadError(#[from] StringReadError),
    #[error("failed to read a variable length enum")]
    LengthReadError(#[from] VariableLengthEnumError),
    #[error("failed to read unspecified data")]
    PureDataReadError(#[from] std::io::Error),
    #[error("failed to read a metadata token")]
    TokenReadError(#[from] MetadataTokenError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataWriteError {
    #[error("failed to write a string")]
    StringWriteError(#[from] StringWriteError),
    #[error("failed to write a variable length enum")]
    LengthWriteError(#[from] VariableLengthEnumError),
    #[error("failed to write unspecified data")]
    PureDataWriteError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataHeaderReadError {
    #[error("failed to read the number of records in metadata")]
    RecordCountError(VariableLengthEnumError),
    #[error("failed to read byte length of metadata records")]
    ByteLengthError(VariableLengthEnumError),
}
