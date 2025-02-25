use std::{i16, io::Read};

use crate::{
    common::traits::private::Sealed,
    v0::{
        expr::{error::NonMatchingExprError, traits::ExprObj, Expr},
        tokens::ExprToken,
        traits::ReadFromWithLength,
    },
};

/// [Unsigned integer literal expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Integer%20Literal.md) in FEF.
///
/// Represents all unsigned integer literals in FEF.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprUnsignedIntLiteral<S: Sized> {
    _marker: std::marker::PhantomData<S>,
    pub(crate) value: u64,
}

/// [Signed integer literal expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Integer%20Literal.md) in FEF.
///
/// Represents all signed integer literals in FEF.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprSignedIntLiteral<S: Sized> {
    _marker: std::marker::PhantomData<S>,
    pub(crate) value: i64,
}
/// Implementation of `from<int>` traits for integer literals.
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

/// Implementation of `try_into<int>` traits for integer literals.
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

impl<S: Sized> Sealed for ExprUnsignedIntLiteral<S> {}
impl<S: Sized> Sealed for ExprSignedIntLiteral<S> {}

impl<R: ?Sized + Read, S: Sized> ReadFromWithLength<R> for ExprSignedIntLiteral<S> {
    type ReadError = std::io::Error;
    /// Reads a signed integer literal from the given reader with the given byte length.
    ///
    /// # Panics
    /// Panics when byte_length is not 1, 2, 4 or 8
    fn read_from<C: ?Sized + crate::v0::config::Config>(
        reader: &mut R,
        _configuration: &C,
        byte_length: usize,
    ) -> Result<Self, Self::ReadError> {
        match byte_length {
            1 => {
                let mut buffer = [0u8; 1];
                reader.read_exact(&mut buffer)?;
                Ok(i8::from_le_bytes(buffer).into())
            }
            2 => {
                let mut buffer = [0u8; 2];
                reader.read_exact(&mut buffer)?;
                Ok(i16::from_le_bytes(buffer).into())
            }
            4 => {
                let mut buffer = [0u8; 4];
                reader.read_exact(&mut buffer)?;
                Ok(i32::from_le_bytes(buffer).into())
            }
            8 => {
                let mut buffer = [0u8; 8];
                reader.read_exact(&mut buffer)?;
                Ok(i64::from_le_bytes(buffer).into())
            }
            _ => panic!(
                "Invalid byte length for signed integer literal reading in ReadFromWithLength: {}",
                byte_length
            ),
        }
    }
}

impl<R: ?Sized + Read, S: Sized> ReadFromWithLength<R> for ExprUnsignedIntLiteral<S> {
    type ReadError = std::io::Error;
    /// Reads an unsigned integer literal from the given reader with the given byte length.
    ///
    /// # Panics
    /// Panics when byte_length is not 1, 2, 4 or 8
    fn read_from<C: ?Sized + crate::v0::config::Config>(
        reader: &mut R,
        _configuration: &C,
        byte_length: usize,
    ) -> Result<Self, Self::ReadError> {
        match byte_length {
            1 => {
                let mut buffer = [0u8; 1];
                reader.read_exact(&mut buffer)?;
                Ok(u8::from_le_bytes(buffer).into())
            }
            2 => {
                let mut buffer = [0u8; 2];
                reader.read_exact(&mut buffer)?;
                Ok(u16::from_le_bytes(buffer).into())
            }
            4 => {
                let mut buffer = [0u8; 4];
                reader.read_exact(&mut buffer)?;
                Ok(u32::from_le_bytes(buffer).into())
            }
            8 => {
                let mut buffer = [0u8; 8];
                reader.read_exact(&mut buffer)?;
                Ok(u64::from_le_bytes(buffer).into())
            }
            _ => panic!("Invalid byte length for unsigned integer literal reading in ReadFromWithLength: {}", byte_length),
        }
    }
}

impl<S: Sized> Into<Expr<S>> for ExprUnsignedIntLiteral<S> {
    fn into(self) -> Expr<S> {
        Expr::UnsignedIntLiteral(self)
    }
}

impl<S: Sized> Into<Expr<S>> for ExprSignedIntLiteral<S> {
    fn into(self) -> Expr<S> {
        Expr::SignedIntLiteral(self)
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprUnsignedIntLiteral<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::UnsignedIntLiteral(value) => Ok(value),
            _ => Err(NonMatchingExprError {
                expected: vec![
                    ExprToken::UnsignedIntLiteral8,
                    ExprToken::UnsignedIntLiteral16,
                    ExprToken::UnsignedIntLiteral32,
                    ExprToken::UnsignedIntLiteral64,
                ],
                found: value.token(),
            }),
        }
    }
}

impl<S: Sized> TryFrom<Expr<S>> for ExprSignedIntLiteral<S> {
    type Error = NonMatchingExprError;

    fn try_from(value: Expr<S>) -> Result<Self, Self::Error> {
        match value {
            Expr::SignedIntLiteral(value) => Ok(value),
            _ => Err(NonMatchingExprError {
                expected: vec![
                    ExprToken::SignedIntLiteral8,
                    ExprToken::SignedIntLiteral16,
                    ExprToken::SignedIntLiteral32,
                    ExprToken::SignedIntLiteral64,
                ],
                found: value.token(),
            }),
        }
    }
}

const U8_MIN: u64 = u8::MIN as u64;
const U8_MAX: u64 = u8::MAX as u64;
const U16_MIN: u64 = u16::MIN as u64;
const U16_MAX: u64 = u16::MAX as u64;
const U32_MIN: u64 = u32::MIN as u64;
const U32_MAX: u64 = u32::MAX as u64;
const U64_MIN: u64 = u64::MIN;
const U64_MAX: u64 = u64::MAX;
impl<S: Sized> ExprObj<S> for ExprUnsignedIntLiteral<S> {
    fn token(&self) -> ExprToken {
        match self.value {
            U8_MIN..=U8_MAX => ExprToken::UnsignedIntLiteral8,
            U16_MIN..=U16_MAX => ExprToken::UnsignedIntLiteral16,
            U32_MIN..=U32_MAX => ExprToken::UnsignedIntLiteral32,
            U64_MIN..=U64_MAX => ExprToken::UnsignedIntLiteral64,
        }
    }
}

const I8_MIN: i64 = i8::MIN as i64;
const I8_MAX: i64 = i8::MAX as i64;
const I16_MIN: i64 = i16::MIN as i64;
const I16_MAX: i64 = i16::MAX as i64;
const I32_MIN: i64 = i32::MIN as i64;
const I32_MAX: i64 = i32::MAX as i64;
const I64_MIN: i64 = i64::MIN;
const I64_MAX: i64 = i64::MAX;
impl<S: Sized> ExprObj<S> for ExprSignedIntLiteral<S> {
    fn token(&self) -> ExprToken {
        match self.value {
            I8_MIN..=I8_MAX => ExprToken::SignedIntLiteral8,
            I16_MIN..=I16_MAX => ExprToken::SignedIntLiteral16,
            I32_MIN..=I32_MAX => ExprToken::SignedIntLiteral32,
            I64_MIN..=I64_MAX => ExprToken::SignedIntLiteral64,
        }
    }
}
