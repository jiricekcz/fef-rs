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
pub struct ExprAddition<S: Sized> {
    lhs: S,
    rhs: S,
}

impl<S: Sized> From<(S, S)> for ExprAddition<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

impl<S: Sized> Into<(S, S)> for ExprAddition<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprAddition<S> {}

impl<S: Sized> Into<Expr<S>> for ExprAddition<S> {
    fn into(self) -> Expr<S> {
        Expr::Addition(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprAddition<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Addition(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Addition,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprAddition<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Addition
    }
}

impl<S: Sized> BinaryOperationExpr<S> for ExprAddition<S> {
    fn lhs(&self) -> &S {
        &self.lhs
    }

    fn rhs(&self) -> &S {
        &self.rhs
    }
}
