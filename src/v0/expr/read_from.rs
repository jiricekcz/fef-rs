//! Implementations of the [TryReadFromWithComposer] for all expressions.
//!
//! This module contains implementations of the [TryReadFromWithComposer] trait for all expressions.
//! When this trait is called on an expression object, it should assume, that the corresponding expression
//! token has already been read from the input stream and the reader is positioned at the beginning of the
//! expression body.
//!
//! This module should never leak any implementation details. It will probably be very macro heavy and
//! subject to a lot of refactoring.
use std::io::Read;

use crate::v0::{
    config::Config,
    expr::error::ExprReadError,
    raw::{Float, Integer, VariableLengthEnum},
    tokens::ExprToken,
    traits::ReadFrom,
};

use super::{
    error::ExprReadWithComposerError,
    traits::{Composer, TryReadFromWithComposer},
    Expr, ExprAddition, ExprCube, ExprCubeRoot, ExprDivision, ExprFalseLiteral, ExprFloatLiteral,
    ExprIntDivision, ExprIntLiteral, ExprIntRoot, ExprModulo, ExprMultiplication, ExprNegation,
    ExprPower, ExprReciprocal, ExprRoot, ExprSquare, ExprSquareRoot, ExprSubtraction,
    ExprTrueLiteral, ExprVariable,
};

macro_rules! impl_read_from_pure_expr {
    ($compose_function_name:ident, $compose_type:ty) => {
        impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
            TryReadFromWithComposer<R, S, C, CP> for $compose_type
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
    TryReadFromWithComposer<R, S, C, CP> for ExprIntLiteral<S>
{
    fn try_read_with_composer(
        byte_stream: &mut R,
        config: &C,
        composer: &mut CP,
    ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
        let value =
            Integer::read_from(byte_stream, config).map_err(|error| ExprReadError::from(error))?;
        let expr = ExprIntLiteral::from(value);
        let composed = composer.compose_int_literal(expr);
        match composed {
            Ok(value) => Ok(value),
            Err(error) => Err(ExprReadWithComposerError::ComposeError(error)),
        }
    }
}

impl<R: ?Sized + Read, S: Sized, C: ?Sized + Config, CP: ?Sized + Composer<S>>
    TryReadFromWithComposer<R, S, C, CP> for ExprFloatLiteral<S>
{
    fn try_read_with_composer(
        byte_stream: &mut R,
        config: &C,
        composer: &mut CP,
    ) -> Result<S, ExprReadWithComposerError<CP::Error>> {
        let value =
            Float::read_from(byte_stream, config).map_err(|error| ExprReadError::from(error))?;
        let expr = ExprFloatLiteral::from(value);
        let composed = composer.compose_float_literal(expr);
        match composed {
            Ok(value) => Ok(value),
            Err(error) => Err(ExprReadWithComposerError::ComposeError(error)),
        }
    }
}

macro_rules! read_by_token {
    ($ident_token:expr, $byte_stream:expr, $config:expr, $composer:expr, $(($token:ident, $expr_obj:ty)), +) => {
        match $ident_token {
            $(ExprToken::$token => <$expr_obj>::try_read_with_composer($byte_stream, $config, $composer)?.into()),+,
        }
    };
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
        Ok(read_by_token!(
            token,
            byte_stream,
            config,
            composer,
            (TrueLiteral, ExprTrueLiteral<S>),
            (FalseLiteral, ExprFalseLiteral<S>),
            (Variable, ExprVariable<S>),
            (IntLiteral, ExprIntLiteral<S>),
            (FloatLiteral, ExprFloatLiteral<S>),
            (Addition, ExprAddition<S>),
            (Subtraction, ExprSubtraction<S>),
            (Multiplication, ExprMultiplication<S>),
            (Division, ExprDivision<S>),
            (IntDivision, ExprIntDivision<S>),
            (Modulo, ExprModulo<S>),
            (Power, ExprPower<S>),
            (Negation, ExprNegation<S>),
            (Root, ExprRoot<S>),
            (IntRoot, ExprIntRoot<S>),
            (Square, ExprSquare<S>),
            (Cube, ExprCube<S>),
            (SquareRoot, ExprSquareRoot<S>),
            (CubeRoot, ExprCubeRoot<S>),
            (Reciprocal, ExprReciprocal<S>)
        ))
    }
}
