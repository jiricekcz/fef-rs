use std::convert::Infallible;

use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{error::NonMatchingExprError, traits::ExprObj, Expr},
        tokens::ExprToken,
    },
};

/// [Float literal expression (binary 32-bit)](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Float%20Literal.md) in FEF.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinaryFloat32Literal<S: Sized> {
    value: f32,
    _marker: std::marker::PhantomData<S>,
}

/// [Float literal expression (binary 64-bit)](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Float%20Literal.md) in FEF.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinaryFloat64Literal<S: Sized> {
    value: f64,
    _marker: std::marker::PhantomData<S>,
}

impl<S: Sized> From<f32> for ExprBinaryFloat32Literal<S> {
    fn from(value: f32) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S: Sized> From<f64> for ExprBinaryFloat64Literal<S> {
    fn from(value: f64) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S: Sized> TryInto<f32> for ExprBinaryFloat32Literal<S> {
    type Error = Infallible;

    fn try_into(self) -> Result<f32, Self::Error> {
        Ok(self.value)
    }
}

impl<S: Sized> TryInto<f64> for ExprBinaryFloat64Literal<S> {
    type Error = Infallible;

    fn try_into(self) -> Result<f64, Self::Error> {
        Ok(self.value)
    }
}

impl<S: Sized> Sealed for ExprBinaryFloat32Literal<S> {}
impl<S: Sized> Sealed for ExprBinaryFloat64Literal<S> {}

impl<S: Sized> TryFrom<Expr<S>> for ExprBinaryFloat32Literal<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::BinaryFloat32Literal(v) => Ok(v),
            _ => Err(NonMatchingExprError {
                expected: vec![ExprToken::BinaryFloatLiteral32],
                found: value.token(),
            }),
        }
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprBinaryFloat64Literal<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::BinaryFloat64Literal(v) => Ok(v),
            _ => Err(NonMatchingExprError {
                expected: vec![ExprToken::BinaryFloatLiteral64],
                found: value.token(),
            }),
        }
    }
}

impl<S: Sized> Into<Expr<S>> for ExprBinaryFloat32Literal<S> {
    fn into(self) -> Expr<S> {
        Expr::BinaryFloat32Literal(self)
    }
}

impl<S: Sized> Into<Expr<S>> for ExprBinaryFloat64Literal<S> {
    fn into(self) -> Expr<S> {
        Expr::BinaryFloat64Literal(self)
    }
}

impl<S: Sized> ExprObj<S> for ExprBinaryFloat32Literal<S> {
    fn token(&self) -> ExprToken {
        ExprToken::BinaryFloatLiteral32
    }
}

impl<S: Sized> ExprObj<S> for ExprBinaryFloat64Literal<S> {
    fn token(&self) -> ExprToken {
        ExprToken::BinaryFloatLiteral64
    }
}
