use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{
            error::NonMatchingExprError,
            traits::{ExprObj, UnaryOperationExprObj},
            Expr,
        },
        tokens::ExprToken,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprNegation<S: Sized> {
    inner: S,
}

impl<S: Sized> From<S> for ExprNegation<S> {
    fn from(inner: S) -> Self {
        Self { inner }
    }
}

impl<S: Sized> Sealed for ExprNegation<S> {}

impl<S: Sized> Into<Expr<S>> for ExprNegation<S> {
    fn into(self) -> Expr<S> {
        Expr::Negation(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprNegation<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Negation(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Negation,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprNegation<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Negation
    }
}

impl<S: Sized> UnaryOperationExprObj<S> for ExprNegation<S> {
    fn inner(&self) -> &S {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    fn into_inner(self) -> S {
        self.inner
    }
}
