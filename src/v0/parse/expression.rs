use std::io::Read;

use crate::v0::{
    config::Config,
    expr::{
        error::{ComposeError, ExprReadWithComposerError},
        traits::{Composer, ExprObj, TryReadFromWithComposer},
        Expr, ExprTree,
    },
};

pub fn parse_expression<
    R: ?Sized + Read,
    C: ?Sized + Config,
    S: Sized,
    CP: ?Sized + Composer<S>,
>(
    byte_stream: &mut R,
    config: &C,
    composer: &mut CP,
) -> Result<S, ExprReadWithComposerError<CP::Error>> {
    <Expr<S> as TryReadFromWithComposer<R, S, C, CP>>::try_read_with_composer(
        byte_stream,
        config,
        composer,
    )
}

pub fn parse_expression_into_tree<R: ?Sized + Read, C: ?Sized + Config>(
    byte_stream: &mut R,
    config: &C,
) -> Result<ExprTree, ExprReadWithComposerError<std::convert::Infallible>> {
    let mut composer = ExprTreeComposer {};
    parse_expression(byte_stream, config, &mut composer)
}

struct ExprTreeComposer {}
impl Composer<ExprTree> for ExprTreeComposer {
    type Error = std::convert::Infallible;
    fn compose_default<E: ExprObj<ExprTree>>(
        &mut self,
        expr: E,
    ) -> Result<ExprTree, ComposeError<Self::Error>> {
        Ok(ExprTree::from(expr.into()))
    }
}
