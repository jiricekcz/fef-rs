use std::{convert::Infallible, io::Write};

use crate::v0::{
    config::Config,
    expr::{traits::Decomposer, ExprTree},
    file::error::{RawFormulaWriteError, SingleFormulaWriteError},
    metadata::MetadataRecord,
    raw::VariableLengthEnum,
    tokens::FileContentTypeToken,
    traits::WriteTo,
    IMPLEMENTED_SPECIFICATION_VERSION,
};

use super::{
    expression::ExprTreeDecomposer, write_configuration, write_expression, write_metadata,
};

/// Writes a [raw formula](https://github.com/jiricekcz/fef-specification/blob/main/file_content_types/Raw%20Formula.md) file to a byte stream.
///
/// This method writes a formula to stream based on the FEF specification. It writes the version number. This
/// is similar to using the [`write_expression`] function, as it takes a [`Decomposer`] to decompose the formula.
/// If you have the formula stored as an [`ExprTree`], you can use the [`write_expression_tree_as_raw_formula`] function.
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

/// Writes an [`ExprTree`] to a byte stream as a [raw formula](https://github.com/jiricekcz/fef-specification/blob/main/file_content_types/Raw%20Formula.md).
///
/// This method writes a formula to stream based on the FEF specification. It writes the version number. If
/// you need to provide a [`Decomposer`], use the [`write_raw_formula`] function instead.
pub fn write_expression_tree_as_raw_formula<W: ?Sized + Write, C: ?Sized + Config>(
    writer: &mut W,
    tree: &ExprTree,
    configuration: &C,
) -> Result<(), RawFormulaWriteError<Infallible>> {
    let mut decomposer = ExprTreeDecomposer {};
    write_raw_formula(writer, tree, configuration, &mut decomposer)
}

/// Writes a [single formula](https://github.com/jiricekcz/fef-specification/blob/main/file_content_types/Single%20Formula.md) file to a byte stream.
///
/// # Generic Types
///     - `'a` - lifetime of a reference to a metadata record.
///     - `EM` - Error type, that occurs when metadata iterator fails.
///     - `S` - Child expression storage type of the formula.
///     - `W` - Writer type.
///     - `C` - Configuration type.
///     - `CW` - Type of configuration to be written (can be different than the one used to write it)
///     - `MI` - Metadata iterator
///     - `DP` - Decomposer type.
///
/// Most of the time, you  want to use the [`write_metadata_vec_expression_tree_as_single_formula`] function.
pub fn write_single_formula<
    'a,
    EM: std::error::Error,
    S: Sized,
    W: ?Sized + Write,
    C: ?Sized + Config,
    CW: ?Sized + Config,
    MI: Iterator<Item = Result<&'a MetadataRecord, EM>>,
    DP: ?Sized + Decomposer<S>,
>(
    writer: &mut W,
    formula: &S,
    configuration: &C,
    configuration_to_write: &CW,
    metadata_iterator: MI,
    metadata_count: usize,
    metadata_byte_size: usize,
    decomposer: &mut DP,
) -> Result<(), SingleFormulaWriteError<<DP as Decomposer<S>>::Error, EM>> {
    let major_version = IMPLEMENTED_SPECIFICATION_VERSION.major();
    VariableLengthEnum::from(major_version as usize)
        .write_to(writer, configuration)
        .map_err(SingleFormulaWriteError::VersionWriteError)?;

    FileContentTypeToken::SingleFormula.write_to(writer, configuration)?;

    write_configuration(writer, configuration_to_write)?;
    write_metadata(
        writer,
        configuration,
        metadata_iterator,
        metadata_count,
        metadata_byte_size,
    )?;
    write_expression(writer, formula, configuration, decomposer)?;

    Ok(())
}

/// Writes a [single formula](https://github.com/jiricekcz/fef-specification/blob/main/file_content_types/Single%20Formula.md) from the most common in memory representation of its parts.
///
/// Expressions are most often represented as [`ExprTree`], metadata is represented as a [`Vec<MetadataRecord>`].
/// This function writes a single formula to a byte stream based on the FEF specification. It writes the version number.
/// If you need to provide a [`Decomposer`], use the [`write_single_formula`] function instead. The same if
/// you have a different representation of metadata.
pub fn write_metadata_vec_expression_tree_as_single_formula<
    W: ?Sized + Write,
    C: ?Sized + Config,
>(
    writer: &mut W,
    tree: &ExprTree,
    configuration: &C,
    metadata_records: &Vec<MetadataRecord>,
) -> Result<(), SingleFormulaWriteError<Infallible, Infallible>> {
    let mut decomposer = ExprTreeDecomposer {};
    write_single_formula(
        writer,
        tree,
        configuration,
        configuration,
        metadata_records
            .into_iter()
            .map(|record| Result::<&MetadataRecord, Infallible>::Ok(record)),
        metadata_records.len(),
        metadata_records
            .iter()
            .map(MetadataRecord::byte_length)
            .sum(),
        &mut decomposer,
    )
}
