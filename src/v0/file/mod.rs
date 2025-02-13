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

pub enum File {
    RawFormula(RawFormulaFile),
    SingleFormula(SingleFormulaFile),
}

impl Sealed for File {}

impl<R: ?Sized + Read> ReadFrom<R> for File {
    type ReadError = FileReadError;
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
