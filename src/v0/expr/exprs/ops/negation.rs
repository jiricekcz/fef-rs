use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{
            error::NonMatchingExprError,
            traits::{ExprObj, UnaryOperationExpr},
            Expr,
        },
        tokens::ExprToken,
    },
};

/// [Negation expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Negation.md) in FEF.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprNegation<S: Sized> {
    operand: S,
}

/// Creates a negation expression from its operand.
impl<S: Sized> From<S> for ExprNegation<S> {
    fn from(inner: S) -> Self {
        Self { operand: inner }
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
                expected: vec![ExprToken::Negation],
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

impl<S: Sized> AsRef<S> for ExprNegation<S> {
    fn as_ref(&self) -> &S {
        &self.operand
    }
}

impl<S: Sized> AsMut<S> for ExprNegation<S> {
    fn as_mut(&mut self) -> &mut S {
        &mut self.operand
    }
}

impl<S: Sized> UnaryOperationExpr<S> for ExprNegation<S> {
    fn into_inner(self) -> S {
        self.operand
    }
}
