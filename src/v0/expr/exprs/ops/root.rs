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

/// [Root expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Root.md) in FEF.s
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprRoot<S: Sized> {
    lhs: S,
    rhs: S,
}

/// Creates a root expression from its left-hand side and right-hand side.
impl<S: Sized> From<(S, S)> for ExprRoot<S> {
    fn from((lhs, rhs): (S, S)) -> Self {
        Self { lhs, rhs }
    }
}

/// Converts the root expression into its left-hand side and right-hand side.
impl<S: Sized> Into<(S, S)> for ExprRoot<S> {
    fn into(self) -> (S, S) {
        (self.lhs, self.rhs)
    }
}

impl<S: Sized> Sealed for ExprRoot<S> {}

impl<S: Sized> Into<Expr<S>> for ExprRoot<S> {
    fn into(self) -> Expr<S> {
        Expr::Root(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprRoot<S> {
    type Error = NonMatchingExprError;

    fn try_from(expr: Expr<S>) -> Result<Self, Self::Error> {
        match expr {
            Expr::Root(expr) => Ok(expr),
            _ => Err(NonMatchingExprError {
                expected: vec![ExprToken::Root],
                found: expr.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprRoot<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Root
    }
}

impl<S: Sized> BinaryOperationExpr<S> for ExprRoot<S> {
    fn lhs(&self) -> &S {
        &self.lhs
    }

    fn rhs(&self) -> &S {
        &self.rhs
    }

    fn rhs_mut(&mut self) -> &mut S {
        &mut self.rhs
    }

    fn lhs_mut(&mut self) -> &mut S {
        &mut self.lhs
    }
}
