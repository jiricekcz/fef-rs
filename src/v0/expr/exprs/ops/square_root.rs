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
/// [Square root expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Sqare%20Root.md) in FEF.s
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprSquareRoot<S: Sized> {
    operand: S,
}

/// Creates a square root expression from its operand.
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

impl<S: Sized> AsRef<S> for ExprSquareRoot<S> {
    fn as_ref(&self) -> &S {
        &self.operand
    }
}

impl<S: Sized> AsMut<S> for ExprSquareRoot<S> {
    fn as_mut(&mut self) -> &mut S {
        &mut self.operand
    }
}

impl<S: Sized> UnaryOperationExpr<S> for ExprSquareRoot<S> {
    fn into_inner(self) -> S {
        self.operand
    }
}
