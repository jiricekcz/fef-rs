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

/// [Multiplication expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Multiplication.md) in FEF.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprMultiplication<S: Sized> {
    lhs: S,
    rhs: S,
}

/// Creates a multiplication expression from its left-hand side and right-hand side.
impl<S: Sized> From<(S, S)> for ExprMultiplication<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

/// Converts the multiplication expression into its left-hand side and right-hand side.
impl<S: Sized> Into<(S, S)> for ExprMultiplication<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprMultiplication<S> {}

impl<S: Sized> Into<Expr<S>> for ExprMultiplication<S> {
    fn into(self) -> Expr<S> {
        Expr::Multiplication(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprMultiplication<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Multiplication(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: vec![ExprToken::Multiplication],
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprMultiplication<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Multiplication
    }
}

impl<S: Sized> BinaryOperationExpr<S> for ExprMultiplication<S> {
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
