use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{
            error::NonMatchingExprError,
            traits::{BinaryOperationExpr, ExprObj},
            Expr,
        },
        tokens::ExprToken,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprRoot<S: Sized> {
    lhs: S,
    rhs: S,
}

impl<S: Sized> From<(S, S)> for ExprRoot<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

impl<S: Sized> Into<(S, S)> for ExprRoot<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprRoot<S> {}

impl<S: Sized> Into<Expr<S>> for ExprRoot<S> {
    fn into(self) -> Expr<S> {
        Expr::Root(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprRoot<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Root(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Root,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprRoot<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Root
    }
}

impl<S: Sized> BinaryOperationExpr<S> for ExprRoot<S> {
    fn lhs(&self) -> &S {
        &self.lhs
    }

    fn rhs(&self) -> &S {
        &self.rhs
    }
}
