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
pub struct ExprReciprocal<S: Sized> {
    operand: S,
}

impl<S: Sized> From<S> for ExprReciprocal<S> {
    fn from(operand: S) -> Self {
        Self { operand }
    }
}

impl<S: Sized> Sealed for ExprReciprocal<S> {}

impl<S: Sized> Into<Expr<S>> for ExprReciprocal<S> {
    fn into(self) -> Expr<S> {
        Expr::Reciprocal(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprReciprocal<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Reciprocal(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Reciprocal,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprReciprocal<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Reciprocal
    }
}

impl<S: Sized> UnaryOperationExpr<S> for ExprReciprocal<S> {
    fn inner(&self) -> &S {
        &self.operand
    }
}
