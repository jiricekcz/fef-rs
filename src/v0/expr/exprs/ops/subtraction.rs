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
pub struct ExprSubtraction<S: Sized> {
    lhs: S,
    rhs: S,
}

impl<S: Sized> From<(S, S)> for ExprSubtraction<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

impl<S: Sized> Into<(S, S)> for ExprSubtraction<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprSubtraction<S> {}

impl<S: Sized> Into<Expr<S>> for ExprSubtraction<S> {
    fn into(self) -> Expr<S> {
        Expr::Subtraction(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprSubtraction<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Subtraction(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Subtraction,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprSubtraction<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Subtraction
    }
}

impl<S: Sized> BinaryOperationExpr<S> for ExprSubtraction<S> {
    fn lhs(&self) -> &S {
        &self.lhs
    }

    fn rhs(&self) -> &S {
        &self.rhs
    }
}
