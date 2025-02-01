use std::io::Read;

use crate::{
    common::traits::private::Sealed,
    v0::{self as fef, traits::ReadFrom},
};
/// Interpretation of a [VariableLengthEnum](crate::v0::raw::VariableLengthEnum) as an expression identifier.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExprToken {
    Variable = 0x04,
    IntLiteral = 0x08,
    FloatLiteral = 0x09,
    TrueLiteral = 0x0A,
    FalseLiteral = 0x0B,
    Addition = 0x10,
    Subtraction = 0x11,
    Multiplication = 0x12,
    Division = 0x13,
    IntDivision = 0x14,
    Modulo = 0x15,
    Power = 0x16,
    Negation = 0x17,
    Root = 0x18,
    IntRoot = 0x19,
    Square = 0x20,
    Cube = 0x21,
    SquareRoot = 0x22,
    CubeRoot = 0x23,
    Reciprocal = 0x24,
}

impl std::fmt::Display for ExprToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::LowerHex for ExprToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.to_owned() as usize)
    }
}

impl std::fmt::UpperHex for ExprToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.to_owned() as usize)
    }
}

/// A fallible conversion from an arbitrary integer to an expression token
///
/// Fails, if the identifier doesn't represent a known token
///
/// # Example
/// ```rust
/// # use fef::v0::tokens::ExprToken;
/// match ExprToken::try_from(4) {
///     Ok(token) => match token { // identifier `0x04` is a variable identifier
///         ExprToken::Variable => assert!(true),
///         _ => unreachable!()
///     }
///     Err(_) => assert!(false) // This identifier exists, so the operation doesn't fail
/// }
/// ```
///
impl TryFrom<usize> for ExprToken {
    type Error = fef::tokens::error::ExprTokenError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0x04 => Ok(ExprToken::Variable),
            0x08 => Ok(ExprToken::IntLiteral),
            0x09 => Ok(ExprToken::FloatLiteral),
            0x0A => Ok(ExprToken::TrueLiteral),
            0x0B => Ok(ExprToken::FalseLiteral),
            0x10 => Ok(ExprToken::Addition),
            0x11 => Ok(ExprToken::Subtraction),
            0x12 => Ok(ExprToken::Multiplication),
            0x13 => Ok(ExprToken::Division),
            0x14 => Ok(ExprToken::IntDivision),
            0x15 => Ok(ExprToken::Modulo),
            0x16 => Ok(ExprToken::Power),
            0x17 => Ok(ExprToken::Negation),
            0x18 => Ok(ExprToken::Root),
            0x19 => Ok(ExprToken::IntRoot),
            0x20 => Ok(ExprToken::Square),
            0x21 => Ok(ExprToken::Cube),
            0x22 => Ok(ExprToken::SquareRoot),
            0x23 => Ok(ExprToken::CubeRoot),
            0x24 => Ok(ExprToken::Reciprocal),
            _ => Err(
                fef::tokens::error::ExprTokenError::IdentifierNotRecognized {
                    identifier: value.into(),
                },
            ),
        }
    }
}

/// A fallible interpretation of a variable length enum as an expression identifier
///
/// Fails, if the identifier doesn't represent a known token
///
/// # Example
/// ```rust
/// # use fef::v0::tokens::ExprToken;
/// # use fef::raw::VariableLengthEnum;
/// let raw_enum = VariableLengthEnum::from(4);
/// match ExprToken::try_from(raw_enum) {
///     Ok(token) => match token { // identifier `0x04` is a variable identifier
///         ExprToken::Variable => assert!(true),
///         _ => unreachable!()
///     }
///     Err(_) => assert!(false) // This identifier exists, so the operation doesn't fail
/// }
/// ```
///
/// A possible distinct fail condition is when the enum is to large. This is a special condition, that terminates interpretation early, if the value is far out of range.
impl TryFrom<fef::raw::VariableLengthEnum> for ExprToken {
    type Error = fef::tokens::error::ExprTokenError;

    fn try_from(value: fef::raw::VariableLengthEnum) -> Result<Self, Self::Error> {
        let identifier: usize = if let Ok(identifier) = value.clone().try_into() {
            identifier
        } else {
            return Err(fef::tokens::error::ExprTokenError::IdentifierTooLarge {
                identifier: value,
            });
        };

        identifier.try_into()
    }
}

impl Sealed for ExprToken {}

impl<R: ?Sized + Read> ReadFrom<R> for ExprToken {
    type ReadError = fef::tokens::error::ExprTokenReadError;

    fn read_from<C: ?Sized + fef::config::Config>(
        reader: &mut R,
        _configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let identifier = fef::raw::VariableLengthEnum::read_from(reader, _configuration)?;
        let token = identifier.try_into()?;
        Ok(token)
    }
}
