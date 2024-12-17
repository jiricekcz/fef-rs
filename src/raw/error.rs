//! Errors for the raw module.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntegerReadError {
    #[error("encountered error while reading byte stream {source}")]
    StreamError {
        #[from]
        source: std::io::Error,
    },
}
