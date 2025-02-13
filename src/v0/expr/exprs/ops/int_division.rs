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
pub struct ExprIntDivision<S: Sized> {
    lhs: S,
    rhs: S,
}

impl<S: Sized> From<(S, S)> for ExprIntDivision<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

impl<S: Sized> Into<(S, S)> for ExprIntDivision<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprIntDivision<S> {}

impl<S: Sized> Into<Expr<S>> for ExprIntDivision<S> {
    fn into(self) -> Expr<S> {
        Expr::IntDivision(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprIntDivision<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::IntDivision(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::IntDivision,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprIntDivision<S> {
    fn token(&self) -> ExprToken {
        ExprToken::IntDivision
    }
}

impl<S: Sized> BinaryOperationExpr<S> for ExprIntDivision<S> {
    fn lhs(&self) -> &S {
        &self.lhs
    }

    fn rhs(&self) -> &S {
        &self.rhs
    }
}
