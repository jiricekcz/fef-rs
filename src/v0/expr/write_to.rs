use std::io::Write;

use crate::v0::{config::Config, tokens::ExprToken, traits::WriteTo};

use super::{
    error::{ExprWriteError, ExprWriteWithDecomposerError},
    traits::{
        BinaryOperationExpr, Decomposer, DecompositionRefContainer, EnumExpr, ExprObj, FloatExpr,
        IntExpr, TryWriteToWithDecomposer, UnaryOperationExpr,
    },
    Expr, ExprAddition, ExprCube, ExprCubeRoot, ExprDivision, ExprFalseLiteral, ExprFloatLiteral,
    ExprIntDivision, ExprIntLiteral, ExprIntRoot, ExprModulo, ExprMultiplication, ExprNegation,
    ExprPower, ExprReciprocal, ExprRoot, ExprSquare, ExprSquareRoot, ExprSubtraction,
    ExprTrueLiteral, ExprVariable,
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

impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
    TryWriteToWithDecomposer<W, S, C, DP> for ExprIntLiteral<S>
{
    fn try_write_with_decomposer(
        &self,
        writer: &mut W,
        config: &C,
        _decomposer: &mut DP,
    ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
        let value = self.integer();
        value
            .write_to(writer, config)
            .map_err(|e| ExprWriteError::from(e))?;
        Ok(())
    }
}

impl<W: ?Sized + Write, S: Sized, C: ?Sized + Config, DP: ?Sized + Decomposer<S>>
    TryWriteToWithDecomposer<W, S, C, DP> for ExprFloatLiteral<S>
{
    fn try_write_with_decomposer(
        &self,
        writer: &mut W,
        config: &C,
        _decomposer: &mut DP,
    ) -> Result<(), ExprWriteWithDecomposerError<<DP as Decomposer<S>>::Error>> {
        let value = self.float();
        value
            .write_to(writer, config)
            .map_err(|e| ExprWriteError::from(e))?;
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
            Expr::IntLiteral(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
            Expr::FloatLiteral(expr) => expr.try_write_with_decomposer(writer, config, decomposer),
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
