use std::io::Write;

use crate::v0::{
    config::Config,
    expr::{
        error::ExprWriteWithDecomposerError,
        traits::{Decomposer, DecompositionRefContainer, TryWriteToWithDecomposer},
        Expr, ExprTree,
    },
};

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

struct ExprTreeDecompositionRefContainer<'a> {
    storage_ref: &'a ExprTree,
}
impl<'a> DecompositionRefContainer<'a, ExprTree> for ExprTreeDecompositionRefContainer<'a> {
    fn inner_as_ref(&self) -> &'a Expr<ExprTree> {
        self.storage_ref.inner()
    }
}
struct ExprTreeDecomposer {}
impl Decomposer<ExprTree> for ExprTreeDecomposer {
    type Error = std::convert::Infallible;
    fn decompose_as_ref<'a>(
        &mut self,
        storage_ref: &'a ExprTree,
    ) -> Result<
        impl DecompositionRefContainer<'a, ExprTree>,
        crate::v0::expr::error::DecomposeError<std::convert::Infallible>,
    > {
        Ok(ExprTreeDecompositionRefContainer { storage_ref })
    }
}

pub fn write_expression_tree<W: ?Sized + Write, C: ?Sized + Config>(
    byte_stream: &mut W,
    tree: &ExprTree,
    config: &C,
) -> Result<(), ExprWriteWithDecomposerError<std::convert::Infallible>> {
    let mut decomposer = ExprTreeDecomposer {};
    write_expression(byte_stream, tree, config, &mut decomposer)
}
