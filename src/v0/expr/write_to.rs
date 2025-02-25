use std::io::Write;

use crate::v0::{
    config::Config,
    raw::error::{FloatWriteError, IntegerWriteError},
    tokens::ExprToken,
    traits::WriteTo,
};

use super::{
    error::{ExprWriteError, ExprWriteWithDecomposerError},
    traits::{
        BinaryOperationExpr, Decomposer, DecompositionRefContainer, EnumExpr, ExprObj,
        TryWriteToWithDecomposer, UnaryOperationExpr,
    },
    Expr, ExprAddition, ExprBinaryFloat32Literal, ExprBinaryFloat64Literal, ExprCube, ExprCubeRoot,
    ExprDivision, ExprFalseLiteral, ExprIntDivision, ExprIntRoot, ExprModulo, ExprMultiplication,
    ExprNegation, ExprPower, ExprReciprocal, ExprRoot, ExprSignedIntLiteral, ExprSquare,
    ExprSquareRoot, ExprSubtraction, ExprTrueLiteral, ExprUnsignedIntLiteral, ExprVariable,
};

macro_rules! impl_try_write_to_with_decomposer_for_unary_expr {
    ($expr_type:ty) => {
        impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
            TryWriteToWithDecomposer<W, S, C, DP> for $expr_type
        {
            fn try_write_with_decomposer(
                &self,
                writer: &mut W,
                config: &C,
                decomposer: &mut DP,
            ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
                let child: &S = self.inner();
                let decomposition = decomposer.decompose_as_ref(child)?.inner_as_ref();
                decomposition.try_write_with_decomposer(writer, config, decomposer)?;
                Ok(())
            }
        }
    };
}

impl_try_write_to_with_decomposer_for_unary_expr!(ExprReciprocal<S>);
impl_try_write_to_with_decomposer_for_unary_expr!(ExprNegation<S>);
impl_try_write_to_with_decomposer_for_unary_expr!(ExprSquare<S>);
impl_try_write_to_with_decomposer_for_unary_expr!(ExprSquareRoot<S>);
impl_try_write_to_with_decomposer_for_unary_expr!(ExprCube<S>);
impl_try_write_to_with_decomposer_for_unary_expr!(ExprCubeRoot<S>);

macro_rules! impl_try_write_to_with_decomposer_for_literal_expr {
    ($expr_type:ty) => {
        impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
            TryWriteToWithDecomposer<W, S, C, DP> for $expr_type
        {
            fn try_write_with_decomposer(
                &self,
                _writer: &mut W,
                _config: &C,
                _decomposer: &mut DP,
            ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
                Ok(())
            }
        }
    };
}

impl_try_write_to_with_decomposer_for_literal_expr!(ExprTrueLiteral<S>);
impl_try_write_to_with_decomposer_for_literal_expr!(ExprFalseLiteral<S>);

macro_rules! impl_try_write_to_with_decomposer_for_variable_expr {
    ($expr_type:ty) => {
        impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
            TryWriteToWithDecomposer<W, S, C, DP> for $expr_type
        {
            fn try_write_with_decomposer(
                &self,
                writer: &mut W,
                config: &C,
                _decomposer: &mut DP,
            ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
                let variable = self.variable_length_enum();
                variable
                    .write_to(writer, config)
                    .map_err(|e| ExprWriteError::from(e))?;
                Ok(())
            }
        }
    };
}

impl_try_write_to_with_decomposer_for_variable_expr!(ExprVariable<S>);

macro_rules! impl_try_write_to_with_decomposer_for_binary_expr {
    ($expr_type:ty) => {
        impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
            TryWriteToWithDecomposer<W, S, C, DP> for $expr_type
        {
            fn try_write_with_decomposer(
                &self,
                writer: &mut W,
                config: &C,
                decomposer: &mut DP,
            ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
                let left: &S = self.lhs();
                let right: &S = self.rhs();
                let left_decomposition = decomposer.decompose_as_ref(left)?.inner_as_ref();
                let right_decomposition = decomposer.decompose_as_ref(right)?.inner_as_ref();
                left_decomposition.try_write_with_decomposer(writer, config, decomposer)?;
                right_decomposition.try_write_with_decomposer(writer, config, decomposer)?;
                Ok(())
            }
        }
    };
}

impl_try_write_to_with_decomposer_for_binary_expr!(ExprAddition<S>);
impl_try_write_to_with_decomposer_for_binary_expr!(ExprSubtraction<S>);
impl_try_write_to_with_decomposer_for_binary_expr!(ExprMultiplication<S>);
impl_try_write_to_with_decomposer_for_binary_expr!(ExprDivision<S>);
impl_try_write_to_with_decomposer_for_binary_expr!(ExprIntDivision<S>);
impl_try_write_to_with_decomposer_for_binary_expr!(ExprPower<S>);
impl_try_write_to_with_decomposer_for_binary_expr!(ExprRoot<S>);
impl_try_write_to_with_decomposer_for_binary_expr!(ExprIntRoot<S>);
impl_try_write_to_with_decomposer_for_binary_expr!(ExprModulo<S>);

const U8_MIN: u64 = u8::MIN as u64;
const U8_MAX: u64 = u8::MAX as u64;
const U16_MIN: u64 = u16::MIN as u64;
const U16_MAX: u64 = u16::MAX as u64;
const U32_MIN: u64 = u32::MIN as u64;
const U32_MAX: u64 = u32::MAX as u64;
const U64_MIN: u64 = u64::MIN;
const U64_MAX: u64 = u64::MAX;
impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
    TryWriteToWithDecomposer<W, S, C, DP> for ExprUnsignedIntLiteral<S>
{
    fn try_write_with_decomposer(
        &self,
        writer: &mut W,
        _config: &C,
        _decomposer: &mut DP,
    ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
        match self.value {
            U8_MIN..=U8_MAX => {
                let buffer = (self.value as u8).to_be_bytes();
                writer
                    .write_all(&buffer)
                    .map_err(|e| ExprWriteError::IntegersWriteError(IntegerWriteError::from(e)))?;
                Ok(())
            }
            U16_MIN..=U16_MAX => {
                let buffer = (self.value as u16).to_be_bytes();
                writer
                    .write_all(&buffer)
                    .map_err(|e| ExprWriteError::IntegersWriteError(IntegerWriteError::from(e)))?;
                Ok(())
            }
            U32_MIN..=U32_MAX => {
                let buffer = (self.value as u32).to_be_bytes();
                writer
                    .write_all(&buffer)
                    .map_err(|e| ExprWriteError::IntegersWriteError(IntegerWriteError::from(e)))?;
                Ok(())
            }
            U64_MIN..=U64_MAX => {
                let buffer = self.value.to_be_bytes();
                writer
                    .write_all(&buffer)
                    .map_err(|e| ExprWriteError::IntegersWriteError(IntegerWriteError::from(e)))?;
                Ok(())
            }
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
impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
    TryWriteToWithDecomposer<W, S, C, DP> for ExprSignedIntLiteral<S>
{
    fn try_write_with_decomposer(
        &self,
        writer: &mut W,
        _config: &C,
        _decomposer: &mut DP,
    ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
        match self.value {
            I8_MIN..=I8_MAX => {
                let buffer = (self.value as i8).to_be_bytes();
                writer
                    .write_all(&buffer)
                    .map_err(|e| ExprWriteError::IntegersWriteError(IntegerWriteError::from(e)))?;
                Ok(())
            }
            I16_MIN..=I16_MAX => {
                let buffer = (self.value as i16).to_be_bytes();
                writer
                    .write_all(&buffer)
                    .map_err(|e| ExprWriteError::IntegersWriteError(IntegerWriteError::from(e)))?;
                Ok(())
            }
            I32_MIN..=I32_MAX => {
                let buffer = (self.value as i32).to_be_bytes();
                writer
                    .write_all(&buffer)
                    .map_err(|e| ExprWriteError::IntegersWriteError(IntegerWriteError::from(e)))?;
                Ok(())
            }
            I64_MIN..=I64_MAX => {
                let buffer = self.value.to_be_bytes();
                writer
                    .write_all(&buffer)
                    .map_err(|e| ExprWriteError::IntegersWriteError(IntegerWriteError::from(e)))?;
                Ok(())
            }
        }
    }
}

impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
    TryWriteToWithDecomposer<W, S, C, DP> for ExprBinaryFloat32Literal<S>
{
    fn try_write_with_decomposer(
        &self,
        writer: &mut W,
        _config: &C,
        _decomposer: &mut DP,
    ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
        let buffer = self.value.to_be_bytes();
        writer
            .write_all(&buffer)
            .map_err(|e| ExprWriteError::FloatsWriteError(FloatWriteError::from(e)))?;
        Ok(())
    }
}

impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
    TryWriteToWithDecomposer<W, S, C, DP> for ExprBinaryFloat64Literal<S>
{
    fn try_write_with_decomposer(
        &self,
        writer: &mut W,
        _config: &C,
        _decomposer: &mut DP,
    ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
        let buffer = self.value.to_be_bytes();
        writer
            .write_all(&buffer)
            .map_err(|e| ExprWriteError::FloatsWriteError(FloatWriteError::from(e)))?;
        Ok(())
    }
}

impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
    TryWriteToWithDecomposer<W, S, C, DP> for Expr<S>
{
    fn try_write_with_decomposer(
        &self,
        writer: &mut W,
        config: &C,
        decomposer: &mut DP,
    ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
        let token: ExprToken = self.token();
        token
            .write_to(writer, config)
            .map_err(|e| ExprWriteError::from(e))?;
        match self {
            Expr::UnsignedIntLiteral(expr) => {
                expr.try_write_with_decomposer(writer, config, decomposer)
            }
            Expr::SignedIntLiteral(expr) => {
                expr.try_write_with_decomposer(writer, config, decomposer)
            }
            Expr::BinaryFloat32Literal(expr) => {
                expr.try_write_with_decomposer(writer, config, decomposer)
            }
            Expr::BinaryFloat64Literal(expr) => {
                expr.try_write_with_decomposer(writer, config, decomposer)
            }
            Expr::TrueLiteral(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::FalseLiteral(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Variable(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Addition(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Subtraction(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Multiplication(expr) => {
                expr.try_write_with_decomposer(writer, config, decomposer)
            }
            Expr::Division(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::IntDivision(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Modulo(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Power(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Negation(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Root(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::IntRoot(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Square(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Cube(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::SquareRoot(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::CubeRoot(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::Reciprocal(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
        }
    }
}
