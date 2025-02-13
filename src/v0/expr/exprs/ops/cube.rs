use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{
            error::NonMatchingExprError,
            traits::{ExprObj, UnaryOperationExpr},
            Expr,
        },
        tokens::ExprToken,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprCube<S: Sized> {
    operand: S,
}

impl<S: Sized> From<S> for ExprCube<S> {
    fn from(operand: S) -> Self {
        Self { operand }
    }
}

impl<S: Sized> Sealed for ExprCube<S> {}

impl<S: Sized> Into<Expr<S>> for ExprCube<S> {
    fn into(self) -> Expr<S> {
        Expr::Cube(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprCube<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Cube(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Cube,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprCube<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Cube
    }
}

impl<S: Sized> UnaryOperationExpr<S> for ExprCube<S> {
    fn inner(&self) -> &S {
        &self.operand
    }
}
