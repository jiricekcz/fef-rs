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
pub struct ExprPower<S: Sized> {
    lhs: S,
    rhs: S,
}

impl<S: Sized> From<(S, S)> for ExprPower<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

impl<S: Sized> Into<(S, S)> for ExprPower<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprPower<S> {}

impl<S: Sized> Into<Expr<S>> for ExprPower<S> {
    fn into(self) -> Expr<S> {
        Expr::Power(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprPower<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Power(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Power,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprPower<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Power
    }
}

impl<S: Sized> BinaryOperationExpr<S> for ExprPower<S> {
    fn lhs(&self) -> &S {
        &self.lhs
    }

    fn rhs(&self) -> &S {
        &self.rhs
    }

    fn lhs_mut(&mut self) -> &mut S {
        &mut self.lhs
    }

    fn rhs_mut(&mut self) -> &mut S {
        &mut self.rhs
    }
}
