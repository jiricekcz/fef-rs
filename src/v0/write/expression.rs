use std::io::Write;

use crate::v0::{
    config::Config,
    expr::{
        error::ExprWriteWithDecomposerError,
        traits::{Decomposer, DecompositionRefContainer, TryWriteToWithDecomposer},
        ExprTree,
    },
};

/// Writes an [expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Expression.md) to a byte stream.
///
/// This function writes an expression to a byte stream using a [`Decomposer`] to decompose children of expressions
/// into their respective parts. This is useful when writing expressions that are not in memory trees, but are
/// instead stored in a different format. Most of the time, you will want to use the [`write_expression_tree`] function,
/// which writes an [`ExprTree`] to a byte stream.
pub fn write_expression<
    W: ?Sized + Write,
    S: Sized,
    C: ?Sized + Config,
    DP: ?Sized + Decomposer<S>,
>(
    byte_stream: &mut W,
    to_write: &S,
    config: &C,
    decomposer: &mut DP,
) -> Result<(), ExprWriteWithDecomposerError<DP::Error>> {
    let expr = decomposer.decompose_as_ref(to_write)?.inner_as_ref();
    expr.try_write_with_decomposer(byte_stream, config, decomposer)?;
    Ok(())
}

pub(crate) struct ExprTreeDecomposer {}
impl Decomposer<ExprTree> for ExprTreeDecomposer {
    type Error = std::convert::Infallible;
    fn decompose_as_ref<'a>(
        &mut self,
        storage_ref: &'a ExprTree,
    ) -> Result<
        impl DecompositionRefContainer<'a, ExprTree>,
        crate::v0::expr::error::DecomposeError<std::convert::Infallible>,
    > {
        Ok(storage_ref.inner())
    }
}

/// Writes an [`ExprTree`] to a byte stream.
///
/// This function is a convenience function that simplifies calling [`write_expression`] with a decomposer that decomposes
/// an [`ExprTree`]. In most cases, you will want to use this function.
///
/// # Example
///
/// Writing the pythagorean theorem expression:
/// ```rust
/// # use fef::v0::write::write_expression_tree;
/// # use fef::v0::config::DEFAULT_CONFIG;
/// # use fef::v0::expr::ExprTree;
/// # use fef::v0::expr::Expr;
/// # use fef::v0::expr::ExprVariable;
/// # use fef::v0::expr::ExprSquare;
/// # use fef::v0::expr::ExprAddition;
/// # use fef::v0::expr::ExprSquareRoot;
/// # use fef::v0::raw::VariableLengthEnum;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let a: ExprTree = Expr::<ExprTree>::Variable(ExprVariable::from(VariableLengthEnum::from(0)).into()).into();
/// let b: ExprTree = Expr::<ExprTree>::Variable(ExprVariable::from(VariableLengthEnum::from(1)).into()).into();
///
/// let a_squared: ExprTree = Expr::<ExprTree>::Square(ExprSquare::from(a).into()).into();
/// let b_squared: ExprTree = Expr::<ExprTree>::Square(ExprSquare::from(b).into()).into();
///
/// let c_squared: ExprTree = Expr::<ExprTree>::Addition(ExprAddition::from((a_squared, b_squared)).into()).into();
/// let c: ExprTree = Expr::<ExprTree>::SquareRoot(ExprSquareRoot::from((c_squared)).into()).into();
///
///
/// let mut writer = Vec::new();
/// write_expression_tree(&mut writer, &c, &DEFAULT_CONFIG)?;
///
/// let expected_bytes: Vec<u8> = vec![
///     0x22, // Square root
///         0x10, // Add  
///             0x20, // Square
///                 0x04, 0x00, // Variable 0 (a)
///             0x20, // Square
///                 0x04, 0x01, // Variable 1 (b)
/// ];
///
/// assert_eq!(writer, expected_bytes);
/// # Ok(())
/// # }
pub fn write_expression_tree<W: ?Sized + Write, C: ?Sized + Config>(
    byte_stream: &mut W,
    tree: &ExprTree,
    config: &C,
) -> Result<(), ExprWriteWithDecomposerError<std::convert::Infallible>> {
    let mut decomposer = ExprTreeDecomposer {};
    write_expression(byte_stream, tree, config, &mut decomposer)
}
