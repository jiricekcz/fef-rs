//! Errors for the raw module.

use thiserror::Error;

/// Errors that can occur while reading an integer from a byte stream.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IntegerReadError {
    /// An io error occurred while reading the byte stream.
    #[error("encountered error while reading byte stream {source}")]
    StreamError {
        #[from]
        source: std::io::Error,
    },
}

/// Errors that can occur while reading a float from a byte stream.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FloatReadError {
    /// An io error occurred while reading the byte stream.
    #[error("encountered error while reading byte stream {source}")]
    StreamError {
        #[from]
        source: std::io::Error,
    },
}

/// Errors that can occur while reading a string from a byte stream.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StringReadError {
    /// An io error occurred while reading the length of the string.
    #[error("encountered error while reading byte stream {source}")]
    LengthReadingError {
        #[from]
        source: std::io::Error,
    },

    #[error("encountered error while processing string length {source}")]
    LengthParsingError {
        #[from]
        source: VariableLengthEnumError,
    },

    /// An io error occurred while reading the string.
    #[error("bytes are not valid utf-8 {source}")]
    InvalidUtf8 {
        #[from]
        source: std::string::FromUtf8Error,
    },

    /// The length of the string is larger than `usize::MAX`.
    #[error("string length is too large")]
    LengthTooLarge,
}

/// Errors that occur while using variable length enums.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum VariableLengthEnumError {
    /// A fallible conversion to a numeric type failed, because the value was too large to fit this type.
    #[error("value is too large to fit in the target type")]
    TooBig,

    #[error("encountered error while reading byte stream {source}")]
    StreamError {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FloatWriteError {
    #[error("encountered error while writing byte stream {source}")]
    StreamError {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IntegerWriteError {
    #[error("encountered error while writing byte stream {source}")]
    StreamError {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StringWriteError {
    #[error("encountered error while writing byte stream {source}")]
    StreamError {
        #[from]
        source: std::io::Error,
    },

    #[error("encountered error while manipulating string length {source}")]
    StringLengthError {
        #[from]
        source: VariableLengthEnumError,
    },
}
