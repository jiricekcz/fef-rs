use std::{convert::Infallible, io::Write};

use crate::v0::{
    config::Config,
    expr::{traits::Decomposer, ExprTree},
    file::error::RawFormulaWriteError,
    raw::VariableLengthEnum,
    tokens::FileContentTypeToken,
    traits::WriteTo,
    IMPLEMENTED_SPECIFICATION_VERSION,
};

use super::{expression::ExprTreeDecomposer, write_expression};

pub fn write_raw_formula<
    S: Sized,
    W: ?Sized + Write,
    C: ?Sized + Config,
    DP: ?Sized + Decomposer<S>,
>(
    writer: &mut W,
    formula: &S,
    configuration: &C,
    decomposer: &mut DP,
) -> Result<(), RawFormulaWriteError<<DP as Decomposer<S>>::Error>> {
    let major_version = IMPLEMENTED_SPECIFICATION_VERSION.major();
    VariableLengthEnum::from(major_version as usize)
        .write_to(writer, configuration)
        .map_err(RawFormulaWriteError::VersionWriteError)?;

    FileContentTypeToken::RawFormula.write_to(writer, configuration)?;

    write_expression(writer, formula, configuration, decomposer)?;
    Ok(())
}

pub fn write_expression_tree_as_raw_formula<W: ?Sized + Write, C: ?Sized + Config>(
    writer: &mut W,
    tree: &ExprTree,
    configuration: &C,
) -> Result<(), RawFormulaWriteError<Infallible>> {
    let mut decomposer = ExprTreeDecomposer {};
    write_raw_formula(writer, tree, configuration, &mut decomposer)
}
