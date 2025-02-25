//! Implementations of the [TryReadFromWithComposer] for all expressions.
//!
//! This module contains implementations of the [TryReadFromWithComposer] trait for all expressions.
//! When this trait is called on an expression object, it should assume, that the corresponding expression
//! token has already been read from the input stream and the reader is positioned at the beginning of the
//! expression body.
//!
//! This module should never leak any implementation details. It will probably be very macro heavy and
//! subject to a lot of refactoring.
//!
//! It is imperative to keep the garbage code contained and private as an implementation detail.
//! It is possible future updates to rust will allow for more concise implementations.
use std::io::Read;

use crate::v0::{
    config::Config, expr::error::ExprReadError, raw::VariableLengthEnum, tokens::ExprToken,
    traits::ReadFrom,
};

use super::{
    error::ExprReadWithComposerError,
    traits::{
        BinaryOperationExpr, Composer, PureExpr, TryReadFromWithComposer,
        TryReadFromWithComposerAndLength, UnaryOperationExpr,
    },
    Expr, ExprAddition, ExprBinaryFloat32Literal, ExprBinaryFloat64Literal, ExprCube, ExprCubeRoot,
    ExprDivision, ExprFalseLiteral, ExprIntDivision, ExprIntRoot, ExprModulo, ExprMultiplication,
    ExprNegation, ExprPower, ExprReciprocal, ExprRoot, ExprSignedIntLiteral, ExprSquare,
    ExprSquareRoot, ExprSubtraction, ExprTrueLiteral, ExprUnsignedIntLiteral, ExprVariable,
};

macro_rules! impl_read_from_pure_expr {
    ($compose_function_name:ident, $compose_type:ty) => {
        impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
            TryReadFromWithComposer<R, S, C, CP> for $compose_type
        where
            $compose_type: PureExpr<S>,
        {
            fn try_read_with_composer(
                _byte_stream: &mut R,
                _config: &C,
                composer: &mut CP,
            ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
                let value = <$compose_type>::from(());
                let composed = composer.$compose_function_name(value);
                match composed {
                    Ok(value) => Ok(value),
                    Err(error) => Err(ExprReadWithComposerError::ComposeError(error)),
                }
            }
        }
    };
}

macro_rules! impl_read_from_enum_expr {
    ($compose_function_name:ident, $compose_type:ty) => {
        impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
            TryReadFromWithComposer<R, S, C, CP> for $compose_type
        {
            fn try_read_with_composer(
                byte_stream: &mut R,
                config: &C,
                composer: &mut CP,
            ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
                let enum_value = VariableLengthEnum::read_from(byte_stream, config)
                    .map_err(|error| ExprReadError::from(error))?;
                let expr = <$compose_type>::try_from(enum_value)
                    .map_err(|error| ExprReadError::from(error))?;
                let composed = composer.$compose_function_name(expr);
                match composed {
                    Ok(value) => Ok(value),
                    Err(error) => Err(ExprReadWithComposerError::ComposeError(error)),
                }
            }
        }
    };
}

macro_rules! impl_read_from_unary_expr {
    ($compose_function_name:ident, $compose_type:ty) => {
        impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
            TryReadFromWithComposer<R, S, C, CP> for $compose_type
        where
            $compose_type: UnaryOperationExpr<S>,
        {
            fn try_read_with_composer(
                byte_stream: &mut R,
                config: &C,
                composer: &mut CP,
            ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
                let inner_expr = Expr::<S>::try_read_with_composer(byte_stream, config, composer)?;
                let expr = <$compose_type>::from(inner_expr);
                let composed = composer.$compose_function_name(expr);
                match composed {
                    Ok(value) => Ok(value),
                    Err(error) => Err(ExprReadWithComposerError::ComposeError(error)),
                }
            }
        }
    };
}

macro_rules! impl_read_from_binary_expr {
    ($compose_function_name:ident, $compose_type:ty) => {
        impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
            TryReadFromWithComposer<R, S, C, CP> for $compose_type
        where
            $compose_type: BinaryOperationExpr<S>,
        {
            fn try_read_with_composer(
                byte_stream: &mut R,
                config: &C,
                composer: &mut CP,
            ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
                let lhs = Expr::<S>::try_read_with_composer(byte_stream, config, composer)?;
                let rhs = Expr::<S>::try_read_with_composer(byte_stream, config, composer)?;
                let expr = <$compose_type>::from((lhs, rhs));
                let composed = composer.$compose_function_name(expr);
                match composed {
                    Ok(value) => Ok(value),
                    Err(error) => Err(ExprReadWithComposerError::ComposeError(error)),
                }
            }
        }
    };
}

impl_read_from_pure_expr!(compose_true_literal, ExprTrueLiteral<S>);
impl_read_from_pure_expr!(compose_false_literal, ExprFalseLiteral<S>);

impl_read_from_enum_expr!(compose_variable, ExprVariable<S>);

impl_read_from_unary_expr!(compose_negation, ExprNegation<S>);
impl_read_from_unary_expr!(compose_square_root, ExprSquareRoot<S>);
impl_read_from_unary_expr!(compose_square, ExprSquare<S>);
impl_read_from_unary_expr!(compose_cube_root, ExprCubeRoot<S>);
impl_read_from_unary_expr!(compose_cube, ExprCube<S>);
impl_read_from_unary_expr!(compose_reciprocal, ExprReciprocal<S>);

impl_read_from_binary_expr!(compose_addition, ExprAddition<S>);
impl_read_from_binary_expr!(compose_subtraction, ExprSubtraction<S>);
impl_read_from_binary_expr!(compose_multiplication, ExprMultiplication<S>);
impl_read_from_binary_expr!(compose_division, ExprDivision<S>);
impl_read_from_binary_expr!(compose_int_division, ExprIntDivision<S>);
impl_read_from_binary_expr!(compose_int_root, ExprIntRoot<S>);
impl_read_from_binary_expr!(compose_root, ExprRoot<S>);
impl_read_from_binary_expr!(compose_power, ExprPower<S>);
impl_read_from_binary_expr!(compose_modulo, ExprModulo<S>);

impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
    TryReadFromWithComposerAndLength<R, S, C, CP> for ExprUnsignedIntLiteral<S>
{
    /// Reads an unsigned integer literal from the byte stream.
    ///
    /// # Panics
    /// Panics, if `byte_length` is not 1, 2, 4, 8
    fn try_read_with_composer(
        byte_stream: &mut R,
        _config: &C,
        composer: &mut CP,
        byte_length: usize,
    ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
        let int: u64 = match byte_length {
            1 => {
                let mut bytes = [0u8; 1];
                byte_stream
                    .read_exact(&mut bytes)
                    .map_err(|error| ExprReadError::from(error))?;
                u8::from_be_bytes(bytes) as u64
            }
            2 => {
                let mut bytes = [0u8; 2];
                byte_stream
                    .read_exact(&mut bytes)
                    .map_err(|error| ExprReadError::from(error))?;
                u16::from_be_bytes(bytes) as u64
            }
            4 => {
                let mut bytes = [0u8; 4];
                byte_stream
                    .read_exact(&mut bytes)
                    .map_err(|error| ExprReadError::from(error))?;
                u32::from_be_bytes(bytes) as u64
            }
            8 => {
                let mut bytes = [0u8; 8];
                byte_stream
                    .read_exact(&mut bytes)
                    .map_err(|error| ExprReadError::from(error))?;
                u64::from_be_bytes(bytes)
            }
            _ => panic!(
                "Invalid byte length for unsigned integer literal {} while reading with composer",
                byte_length
            ),
        };
        let expr: ExprUnsignedIntLiteral<S> = ExprUnsignedIntLiteral::from(int);
        Ok(composer.compose_unsigned_int_literal(expr)?)
    }
}

impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
    TryReadFromWithComposerAndLength<R, S, C, CP> for ExprSignedIntLiteral<S>
{
    /// Reads an unsigned integer literal from the byte stream.
    ///
    /// # Panics
    /// Panics, if `byte_length` is not 1, 2, 4, 8
    fn try_read_with_composer(
        byte_stream: &mut R,
        _config: &C,
        composer: &mut CP,
        byte_length: usize,
    ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
        let int: i64 = match byte_length {
            1 => {
                let mut bytes = [0u8; 1];
                byte_stream
                    .read_exact(&mut bytes)
                    .map_err(|error| ExprReadError::from(error))?;
                i8::from_be_bytes(bytes) as i64
            }
            2 => {
                let mut bytes = [0u8; 2];
                byte_stream
                    .read_exact(&mut bytes)
                    .map_err(|error| ExprReadError::from(error))?;
                i16::from_be_bytes(bytes) as i64
            }
            4 => {
                let mut bytes = [0u8; 4];
                byte_stream
                    .read_exact(&mut bytes)
                    .map_err(|error| ExprReadError::from(error))?;
                i32::from_be_bytes(bytes) as i64
            }
            8 => {
                let mut bytes = [0u8; 8];
                byte_stream
                    .read_exact(&mut bytes)
                    .map_err(|error| ExprReadError::from(error))?;
                i64::from_be_bytes(bytes)
            }
            _ => panic!(
                "Invalid byte length for unsigned integer literal {} while reading with composer",
                byte_length
            ),
        };
        let expr: ExprSignedIntLiteral<S> = ExprSignedIntLiteral::from(int);
        Ok(composer.compose_signed_int_literal(expr)?)
    }
}

impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
    TryReadFromWithComposer<R, S, C, CP> for ExprBinaryFloat32Literal<S>
{
    fn try_read_with_composer(
        byte_stream: &mut R,
        _config: &C,
        composer: &mut CP,
    ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
        let mut bytes = [0u8; 4];
        byte_stream
            .read_exact(&mut bytes)
            .map_err(|error| ExprReadError::from(error))?;
        let float = f32::from_be_bytes(bytes);
        let expr: ExprBinaryFloat32Literal<S> = ExprBinaryFloat32Literal::from(float);
        Ok(composer.compose_binary_float_32_literal(expr)?)
    }
}

impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
    TryReadFromWithComposer<R, S, C, CP> for ExprBinaryFloat64Literal<S>
{
    fn try_read_with_composer(
        byte_stream: &mut R,
        _config: &C,
        composer: &mut CP,
    ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
        let mut bytes = [0u8; 8];
        byte_stream
            .read_exact(&mut bytes)
            .map_err(|error| ExprReadError::from(error))?;
        let float = f64::from_be_bytes(bytes);
        let expr: ExprBinaryFloat64Literal<S> = ExprBinaryFloat64Literal::from(float);
        Ok(composer.compose_binary_float_64_literal(expr)?)
    }
}

impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
    TryReadFromWithComposer<R, S, C, CP> for Expr<S>
{
    fn try_read_with_composer(
        byte_stream: &mut R,
        config: &C,
        composer: &mut CP,
    ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
        let token = ExprToken::read_from(byte_stream, config)
            .map_err(|error| ExprReadError::from(error))?;
        Ok(match token {
            ExprToken::Addition => {
                ExprAddition::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::BinaryFloatLiteral32 => {
                ExprBinaryFloat32Literal::<S>::try_read_with_composer(
                    byte_stream,
                    config,
                    composer,
                )?
                .into()
            }
            ExprToken::BinaryFloatLiteral64 => {
                ExprBinaryFloat64Literal::<S>::try_read_with_composer(
                    byte_stream,
                    config,
                    composer,
                )?
                .into()
            }
            ExprToken::Cube => {
                ExprCube::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::CubeRoot => {
                ExprCubeRoot::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Division => {
                ExprDivision::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::FalseLiteral => {
                ExprFalseLiteral::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::IntDivision => {
                ExprIntDivision::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::IntRoot => {
                ExprIntRoot::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Modulo => {
                ExprModulo::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Multiplication => {
                ExprMultiplication::<S>::try_read_with_composer(byte_stream, config, composer)?
                    .into()
            }
            ExprToken::Negation => {
                ExprNegation::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Power => {
                ExprPower::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Reciprocal => {
                ExprReciprocal::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Root => {
                ExprRoot::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Square => {
                ExprSquare::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::SquareRoot => {
                ExprSquareRoot::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Subtraction => {
                ExprSubtraction::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::TrueLiteral => {
                ExprTrueLiteral::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }
            ExprToken::Variable => {
                ExprVariable::<S>::try_read_with_composer(byte_stream, config, composer)?.into()
            }

            ExprToken::SignedIntLiteral8 => {
                ExprSignedIntLiteral::<S>::try_read_with_composer(byte_stream, config, composer, 1)?
                    .into()
            }
            ExprToken::SignedIntLiteral16 => {
                ExprSignedIntLiteral::<S>::try_read_with_composer(byte_stream, config, composer, 2)?
                    .into()
            }
            ExprToken::SignedIntLiteral32 => {
                ExprSignedIntLiteral::<S>::try_read_with_composer(byte_stream, config, composer, 4)?
                    .into()
            }
            ExprToken::SignedIntLiteral64 => {
                ExprSignedIntLiteral::<S>::try_read_with_composer(byte_stream, config, composer, 8)?
                    .into()
            }

            ExprToken::UnsignedIntLiteral8 => ExprUnsignedIntLiteral::<S>::try_read_with_composer(
                byte_stream,
                config,
                composer,
                1,
            )?
            .into(),
            ExprToken::UnsignedIntLiteral16 => ExprUnsignedIntLiteral::<S>::try_read_with_composer(
                byte_stream,
                config,
                composer,
                2,
            )?
            .into(),
            ExprToken::UnsignedIntLiteral32 => ExprUnsignedIntLiteral::<S>::try_read_with_composer(
                byte_stream,
                config,
                composer,
                4,
            )?
            .into(),
            ExprToken::UnsignedIntLiteral64 => ExprUnsignedIntLiteral::<S>::try_read_with_composer(
                byte_stream,
                config,
                composer,
                8,
            )?
            .into(),
        })
    }
}
