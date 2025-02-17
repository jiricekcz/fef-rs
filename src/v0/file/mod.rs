//! Handling of the different [file content types](https://github.com/jiricekcz/fef-specification/blob/main/README.md).

pub mod error;
mod raw_formula;
mod read_from;
mod single_formula;

use std::io::Read;

use error::FileReadError;
pub use raw_formula::RawFormulaFile;
pub use single_formula::SingleFormulaFile;

use crate::common::traits::private::Sealed;

use super::{tokens::FileContentTypeToken, traits::ReadFrom};

/// A file as defined by the [FEF specification](https://github.com/jiricekcz/fef-specification/blob/main/README.md).
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum File {
    /// A raw formula file. See [`RawFormulaFile`].
    RawFormula(RawFormulaFile),
    /// A single formula file. See [`SingleFormulaFile`].
    SingleFormula(SingleFormulaFile),
}

impl Sealed for File {}

impl<R: ?Sized + Read> ReadFrom<R> for File {
    type ReadError = FileReadError;

    /// Reads a file from a reader. Expects the version has already been read.
    ///
    /// It reads the file content type token and then reads the file based on the token.
    fn read_from<C: ?Sized + super::config::Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let token = FileContentTypeToken::read_from(reader, configuration)?;
        Ok(match token {
            FileContentTypeToken::RawFormula => {
                File::RawFormula(RawFormulaFile::read_from(reader, configuration)?)
            }
            FileContentTypeToken::SingleFormula => {
                File::SingleFormula(SingleFormulaFile::read_from(reader, configuration)?)
            }
        })
    }
}
