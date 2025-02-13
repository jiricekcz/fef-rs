pub mod error;
mod raw_formula;
mod read_from;
mod single_formula;

pub use raw_formula::RawFormulaFile;
pub use single_formula::SingleFormulaFile;

pub enum File {
    RawFormula(RawFormulaFile),
    SingleFormula(SingleFormulaFile),
}
