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
pub struct ExprSquare<S: Sized> {
    operand: S,
}

impl<S: Sized> From<S> for ExprSquare<S> {
    fn from(operand: S) -> Self {
        Self { operand }
    }
}

impl<S: Sized> Sealed for ExprSquare<S> {}

impl<S: Sized> Into<Expr<S>> for ExprSquare<S> {
    fn into(self) -> Expr<S> {
        Expr::Square(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprSquare<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Square(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Square,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprSquare<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Square
    }
}

impl<S: Sized> UnaryOperationExprObj<S> for ExprSquare<S> {
    fn inner(&self) -> &S {
        &self.operand
    }
    fn inner_mut(&mut self) -> &mut S {
        &mut self.operand
    }
    fn into_inner(self) -> S {
        self.operand
    }
}
