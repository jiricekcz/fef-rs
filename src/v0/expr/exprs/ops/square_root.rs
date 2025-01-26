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
pub struct ExprSquareRoot<S: Sized> {
    operand: S,
}

impl<S: Sized> From<S> for ExprSquareRoot<S> {
    fn from(operand: S) -> Self {
        Self { operand }
    }
}

impl<S: Sized> Sealed for ExprSquareRoot<S> {}

impl<S: Sized> Into<Expr<S>> for ExprSquareRoot<S> {
    fn into(self) -> Expr<S> {
        Expr::SquareRoot(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprSquareRoot<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::SquareRoot(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::SquareRoot,
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprSquareRoot<S> {
    fn token(&self) -> ExprToken {
        ExprToken::SquareRoot
    }
}

impl<S: Sized> UnaryOperationExprObj<S> for ExprSquareRoot<S> {
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
