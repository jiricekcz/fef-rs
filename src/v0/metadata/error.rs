//! Error types for metadata module.
use std::{convert::Infallible, fmt::Debug};

use thiserror::Error;

use crate::v0::{
    raw::error::{StringReadError, StringWriteError, VariableLengthEnumError},
    tokens::error::MetadataTokenError,
};

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataRecordReadError {
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
pub enum MetadataRecordWriteError {
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

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataHeaderWriteError {
    #[error("failed to write the number of records in metadata")]
    RecordCountError(VariableLengthEnumError),
    #[error("failed to write byte length of metadata records")]
    ByteLengthError(VariableLengthEnumError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataWriteError {
    #[error("failed to write metadata header")]
    HeaderError(#[from] MetadataHeaderWriteError),
    #[error("failed to write metadata record")]
    RecordError(#[from] MetadataRecordWriteError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetadataReadError {
    #[error("failed to read metadata header")]
    HeaderError(#[from] MetadataHeaderReadError),
    #[error("failed to read metadata record")]
    RecordError(#[from] MetadataRecordReadError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FromIteratorMetadataWriteError<E: std::error::Error + Debug> {
    #[error("an error occurred in user provided iterator")]
    IteratorError(E),
    #[error("an error occurred while writing metadata")]
    MetadataWriteError(#[from] MetadataWriteError),
}

#[derive(Error, Debug)]
#[error("metadata identifier {identifier} is out of range [{}..={}]", range.start(), range.end())]
pub struct MetadataIdentifierOutOfRangeError {
    pub identifier: u32,
    pub range: std::ops::RangeInclusive<u32>,
}

impl MetadataIdentifierOutOfRangeError {
    pub(crate) fn custom_key(identifier: u32) -> Self {
        Self {
            identifier,
            range: 0x100000..=0x1FFFFF,
        }
    }
}

impl<E: std::error::Error> From<Infallible> for FromIteratorMetadataWriteError<E> {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
