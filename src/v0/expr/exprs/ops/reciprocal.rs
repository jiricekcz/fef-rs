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

/// [Reciprocal expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Reciprocal.md) in FEF.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprReciprocal<S: Sized> {
    operand: S,
}

/// Creates a reciprocal expression from its operand.
impl<S: Sized> From<S> for ExprReciprocal<S> {
    fn from(operand: S) -> Self {
        Self { operand }
    }
}

impl<S: Sized> Sealed for ExprReciprocal<S> {}

impl<S: Sized> Into<Expr<S>> for ExprReciprocal<S> {
    fn into(self) -> Expr<S> {
        Expr::Reciprocal(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprReciprocal<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Reciprocal(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: vec![ExprToken::Reciprocal],
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprReciprocal<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Reciprocal
    }
}

impl<S: Sized> AsRef<S> for ExprReciprocal<S> {
    fn as_ref(&self) -> &S {
        &self.operand
    }
}

impl<S: Sized> AsMut<S> for ExprReciprocal<S> {
    fn as_mut(&mut self) -> &mut S {
        &mut self.operand
    }
}

impl<S: Sized> UnaryOperationExpr<S> for ExprReciprocal<S> {
    fn into_inner(self) -> S {
        self.operand
    }
}
