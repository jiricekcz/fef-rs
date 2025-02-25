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

/// [Cube expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Cube.md) in FEF.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprCube<S: Sized> {
    operand: S,
}

/// Creates a cube expression from its operand.
impl<S: Sized> From<S> for ExprCube<S> {
    fn from(operand: S) -> Self {
        Self { operand }
    }
}

impl<S: Sized> Sealed for ExprCube<S> {}

impl<S: Sized> Into<Expr<S>> for ExprCube<S> {
    fn into(self) -> Expr<S> {
        Expr::Cube(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprCube<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Cube(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: vec![ExprToken::Cube],
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprCube<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Cube
    }
}

impl<S: Sized> AsRef<S> for ExprCube<S> {
    fn as_ref(&self) -> &S {
        &self.operand
    }
}

impl<S: Sized> AsMut<S> for ExprCube<S> {
    fn as_mut(&mut self) -> &mut S {
        &mut self.operand
    }
}

impl<S: Sized> UnaryOperationExpr<S> for ExprCube<S> {
    fn into_inner(self) -> S {
        self.operand
    }
}
