use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{
            error::NonMatchingExprError,
            traits::{EnumExpr, ExprObj},
            Expr,
        },
        raw::VariableLengthEnum,
        tokens::ExprToken,
    },
};

/// [Variable expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Variable.md) in FEF.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprVariable<S: Sized> {
    _phantom: std::marker::PhantomData<S>,
    id: VariableLengthEnum,
}

impl<S: Sized> Sealed for ExprVariable<S> {}

impl<S: Sized> TryFrom<Expr<S>> for ExprVariable<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::Variable(v) => Ok(v),
            _ => Err(NonMatchingExprError {
                expected: ExprToken::Variable,
                found: value.token(),
            }),
        }
    }
}

impl<S: Sized> Into<Expr<S>> for ExprVariable<S> {
    fn into(self) -> Expr<S> {
        Expr::Variable(self)
    }
}

impl<S: Sized> ExprObj<S> for ExprVariable<S> {
    fn token(&self) -> ExprToken {
        ExprToken::Variable
    }
}

impl<S: Sized> From<VariableLengthEnum> for ExprVariable<S> {
    fn from(id: VariableLengthEnum) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            id,
        }
    }
}

impl<S: Sized> Into<VariableLengthEnum> for ExprVariable<S> {
    fn into(self) -> VariableLengthEnum {
        self.id
    }
}

impl<S: Sized> EnumExpr<S> for ExprVariable<S> {}
