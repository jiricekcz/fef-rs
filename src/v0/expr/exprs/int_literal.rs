/// [Unsigned integer literal expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Integer%20Literal.md) in FEF.
///
/// Represents all unsigned integer literals in FEF.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprUnsignedIntLiteral<S: Sized> {
    _marker: std::marker::PhantomData<S>,
    value: u64,
}

/// [Signed integer literal expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Integer%20Literal.md) in FEF.
///
/// Represents all signed integer literals in FEF.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprSignedIntLiteral<S: Sized> {
    _marker: std::marker::PhantomData<S>,
    value: i64,
}
/// Implementation of from<int> traits for integer literals.
///
/// Module is used for strict scoping of the macro enforced by the compiler.
mod from_impl {
    use super::*;
    /// Implements the conversion from any integer type to the integer literal expression.
    macro_rules! implement_from_int {
        ($struct:ty, $from:ty) => {
            impl<S: Sized> From<$from> for $struct {
                fn from(value: $from) -> Self {
                    Self {
                        _marker: std::marker::PhantomData,
                        value: value as _,
                    }
                }
            }
        };
    }

    implement_from_int!(ExprUnsignedIntLiteral<S>, u64);
    implement_from_int!(ExprUnsignedIntLiteral<S>, u32);
    implement_from_int!(ExprUnsignedIntLiteral<S>, u16);
    implement_from_int!(ExprUnsignedIntLiteral<S>, u8);

    implement_from_int!(ExprSignedIntLiteral<S>, i64);
    implement_from_int!(ExprSignedIntLiteral<S>, i32);
    implement_from_int!(ExprSignedIntLiteral<S>, i16);
    implement_from_int!(ExprSignedIntLiteral<S>, i8);
}

/// Implementation of try_into<int> traits for integer literals.
///
/// Module is used for strict scoping of the macro enforced by the compiler.
mod try_into {
    use super::*;
    /// Implements the conversion from integer literal expression to any signed integer type.
    macro_rules! implement_try_into_signed_int {
        ($struct:ty, $into:ty) => {
            impl<S: Sized> TryInto<$into> for $struct {
                type Error = <$into as TryFrom<i64>>::Error;

                fn try_into(self) -> Result<$into, Self::Error> {
                    self.value.try_into()
                }
            }
        };
    }

    implement_try_into_signed_int!(ExprSignedIntLiteral<S>, i64);
    implement_try_into_signed_int!(ExprSignedIntLiteral<S>, i32);
    implement_try_into_signed_int!(ExprSignedIntLiteral<S>, i16);
    implement_try_into_signed_int!(ExprSignedIntLiteral<S>, i8);

    /// Implements the conversion from integer literal expression to any unsigned integer type.
    macro_rules! implement_try_into_unsigned_int {
        ($struct:ty, $into:ty) => {
            impl<S: Sized> TryInto<$into> for $struct {
                type Error = <$into as TryFrom<u64>>::Error;

                fn try_into(self) -> Result<$into, Self::Error> {
                    self.value.try_into()
                }
            }
        };
    }

    implement_try_into_unsigned_int!(ExprUnsignedIntLiteral<S>, u64);
    implement_try_into_unsigned_int!(ExprUnsignedIntLiteral<S>, u32);
    implement_try_into_unsigned_int!(ExprUnsignedIntLiteral<S>, u16);
    implement_try_into_unsigned_int!(ExprUnsignedIntLiteral<S>, u8);
}
