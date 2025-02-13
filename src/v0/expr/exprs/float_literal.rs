use crate::common::traits::private::Sealed;
use crate::v0::expr::error::NonMatchingExprError;
use crate::v0::expr::traits::{ExprObj, FloatExpr};
use crate::v0::expr::Expr;
use crate::v0::raw::Float;
use crate::v0::tokens::ExprToken;

/// Represents a float literal expression in the FEF specification.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprFloatLiteral<S: Sized> {
    value: Float,
    _marker: std::marker::PhantomData<S>,
}

impl<S: Sized> ExprFloatLiteral<S> {
    /// Creates a new float literal expression with the given value.
    pub fn new(value: Float) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the value of the float literal expression.
    pub fn value(&self) -> Float {
        self.value
    }
}

impl<S: Sized> Sealed for ExprFloatLiteral<S> {}

impl<S: Sized> Into<Expr<S>> for ExprFloatLiteral<S> {
    fn into(self) -> Expr<S> {
        Expr::FloatLiteral(self)
    }
}

impl<S: Sized> Into<Float> for ExprFloatLiteral<S> {
    fn into(self) -> Float {
        self.value
    }
}

impl<S: Sized> From<Float> for ExprFloatLiteral<S> {
    fn from(value: Float) -> Self {
        Self::new(value)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprFloatLiteral<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::FloatLiteral(v) => Ok(v),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::FloatLiteral,
                found: value.token(),
            }),
        }
    }
}

impl<S: Sized> ExprObj<S> for ExprFloatLiteral<S> {
    fn token(&self) -> ExprToken {
        ExprToken::FloatLiteral
    }
}

impl<S: Sized> FloatExpr<S> for ExprFloatLiteral<S> {
    fn float(&self) -> &Float {
        &self.value
    }
}
