use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{
            error::NonMatchingExprError,
            traits::{ExprObj, IntExpr},
            Expr,
        },
        raw::Integer,
        tokens::ExprToken,
    },
};

/// [Integer literal expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Integer%20Literal.md) in FEF.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprIntLiteral<S: Sized> {
    _phantom: std::marker::PhantomData<S>,
    value: Integer,
}

impl<S: Sized> Sealed for ExprIntLiteral<S> {}

impl<S: Sized> TryFrom<Expr<S>> for ExprIntLiteral<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::IntLiteral(v) => Ok(v),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::IntLiteral,
                found: value.token(),
            }),
        }
    }
}

impl<S: Sized> Into<Expr<S>> for ExprIntLiteral<S> {
    fn into(self) -> Expr<S> {
        Expr::IntLiteral(self)
    }
}

impl<S: Sized> ExprObj<S> for ExprIntLiteral<S> {
    fn token(&self) -> ExprToken {
        ExprToken::IntLiteral
    }
}

impl<S: Sized> From<Integer> for ExprIntLiteral<S> {
    fn from(value: Integer) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            value,
        }
    }
}

impl<S: Sized> Into<Integer> for ExprIntLiteral<S> {
    fn into(self) -> Integer {
        self.value
    }
}

impl<S: Sized> AsRef<Integer> for ExprIntLiteral<S> {
    fn as_ref(&self) -> &Integer {
        &self.value
    }
}

impl<S: Sized> AsMut<Integer> for ExprIntLiteral<S> {
    fn as_mut(&mut self) -> &mut Integer {
        &mut self.value
    }
}

impl<S: Sized> IntExpr<S> for ExprIntLiteral<S> {}
