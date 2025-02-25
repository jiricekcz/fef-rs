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

/// [Division expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Division.md) in FEF.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprDivision<S: Sized> {
    lhs: S,
    rhs: S,
}

/// Creates a division expression from its left-hand side and right-hand side.
impl<S: Sized> From<(S, S)> for ExprDivision<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

/// Converts the division expression into its left-hand side and right-hand side.
impl<S: Sized> Into<(S, S)> for ExprDivision<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprDivision<S> {}

impl<S: Sized> Into<Expr<S>> for ExprDivision<S> {
    fn into(self) -> Expr<S> {
        Expr::Division(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprDivision<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Division(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: vec![ExprToken::Division],
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprDivision<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Division
    }
}

impl<S: Sized> BinaryOperationExpr<S> for ExprDivision<S> {
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
