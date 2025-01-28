use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{
            error::NonMatchingExprError,
            traits::{ExprObj, PureExpr},
            Expr,
        },
        tokens::ExprToken,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprTrueLiteral<S: Sized> {
    _phantom: std::marker::PhantomData<S>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprFalseLiteral<S: Sized> {
    _phantom: std::marker::PhantomData<S>,
}

impl<S: Sized> Default for ExprTrueLiteral<S> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<S: Sized> Default for ExprFalseLiteral<S> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S: Sized> Sealed for ExprTrueLiteral<S> {}
impl<S: Sized> Sealed for ExprFalseLiteral<S> {}

impl<S: Sized> TryFrom<Expr<S>> for ExprTrueLiteral<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::TrueLiteral(v) => Ok(v),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::TrueLiteral,
                found: value.token(),
            }),
        }
    }
}
impl<S: Sized> TryFrom<Expr<S>> for ExprFalseLiteral<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::FalseLiteral(v) => Ok(v),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::FalseLiteral,
                found: value.token(),
            }),
        }
    }
}

impl<S: Sized> Into<Expr<S>> for ExprTrueLiteral<S> {
    fn into(self) -> Expr<S> {
        Expr::TrueLiteral(self)
    }
}
impl<S: Sized> Into<Expr<S>> for ExprFalseLiteral<S> {
    fn into(self) -> Expr<S> {
        Expr::FalseLiteral(self)
    }
}

impl<S: Sized> ExprObj<S> for ExprTrueLiteral<S> {
    fn token(&self) -> ExprToken {
        ExprToken::TrueLiteral
    }
}
impl<S: Sized> ExprObj<S> for ExprFalseLiteral<S> {
    fn token(&self) -> ExprToken {
        ExprToken::FalseLiteral
    }
}

impl<S: Sized> From<()> for ExprTrueLiteral<S> {
    fn from(_: ()) -> Self {
        Self::default()
    }
}
impl<S: Sized> From<()> for ExprFalseLiteral<S> {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl<S: Sized> PureExpr<S> for ExprTrueLiteral<S> {}
impl<S: Sized> PureExpr<S> for ExprFalseLiteral<S> {}
