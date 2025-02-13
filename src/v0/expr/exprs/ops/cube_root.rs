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
pub struct ExprCubeRoot<S: Sized> {
    operand: S,
}

impl<S: Sized> From<S> for ExprCubeRoot<S> {
    fn from(operand: S) -> Self {
        Self { operand }
    }
}

impl<S: Sized> Sealed for ExprCubeRoot<S> {}

impl<S: Sized> Into<Expr<S>> for ExprCubeRoot<S> {
    fn into(self) -> Expr<S> {
        Expr::CubeRoot(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprCubeRoot<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::CubeRoot(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::CubeRoot,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprCubeRoot<S> {
    fn token(&self) -> ExprToken {
        ExprToken::CubeRoot
    }
}

impl<S: Sized> UnaryOperationExpr<S> for ExprCubeRoot<S> {
    fn inner(&self) -> &S {
        &self.operand
    }
}
