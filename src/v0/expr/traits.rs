use crate::{
    common::traits::private::Sealed,
    v0::{
        raw::{Float, Integer, VariableLengthEnum},
        tokens::ExprToken,
    },
};

use super::Expr;

/// A trait for all expression objects.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between expression objects.
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
pub trait ExprObj<S: Sized>: Sealed + Sized + Into<Expr<S>> + TryFrom<Expr<S>> {
    /// Returns the [token](ExprToken) of the expression.
    fn token(&self) -> ExprToken;

    /// Generalizes this object into the [Expr](crate::v0::expr::Expr) enum type.
    fn into_expr(self) -> Expr<S> {
        self.into()
    }
}

/// All expressions that are fully characterized by a single enum.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// Not all values of a given variable length enum must be valid for the expression,
/// but all expressions must be representable by a single value of the variable length enum.
pub trait EnumExprObj<S: Sized>:
    Sealed + ExprObj<S> + TryFrom<VariableLengthEnum> + Into<VariableLengthEnum>
{
    /// Converts this object into a [VariableLengthEnum](crate::v0::raw::VariableLengthEnum).
    fn into_variable_length_enum(self) -> VariableLengthEnum {
        self.into()
    }
}

/// A trait for all integer expression objects.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between integer expression objects.
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
pub trait IntExprObj<S: Sized>: Sealed + Sized + Into<Integer> + TryFrom<Integer> {
    /// Converts this object into an [Integer](crate::v0::raw::Integer).
    fn into_integer(self) -> Integer {
        self.into()
    }
}

/// A trait for all float expression objects.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between float expression objects.
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
pub trait FloatExprObj<S: Sized>: Sealed + Sized + Into<Float> + TryFrom<Float> {
    fn into_float(self) -> Float {
        self.into()
    }
}

/// A trait for all expression objects that hold no value.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between expression objects that hold no value.
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
pub trait PureExprObj<S: Sized>: Sealed + Sized + From<()> {}
