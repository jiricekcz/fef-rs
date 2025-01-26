use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{
            error::NonMatchingExprError,
            traits::{BinaryOperationExprObj, ExprObj},
            Expr,
        },
        tokens::ExprToken,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprModulo<S: Sized> {
    lhs: S,
    rhs: S,
}

impl<S: Sized> From<(S, S)> for ExprModulo<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

impl<S: Sized> Into<(S, S)> for ExprModulo<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprModulo<S> {}

impl<S: Sized> Into<Expr<S>> for ExprModulo<S> {
    fn into(self) -> Expr<S> {
        Expr::Modulo(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprModulo<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Modulo(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Modulo,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprModulo<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Modulo
    }
}

impl<S: Sized> BinaryOperationExprObj<S> for ExprModulo<S> {
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
