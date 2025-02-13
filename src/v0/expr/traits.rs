use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        expr::{
            error::{ComposeError, DefaultComposeError, ExprReadWithComposerError},
            Expr, ExprTrueLiteral,
        },
        raw::{Float, Integer, VariableLengthEnum},
        tokens::ExprToken,
    },
};

use super::{
    error::{DecomposeError, ExprWriteWithDecomposerError},
    ExprAddition, ExprCube, ExprCubeRoot, ExprDivision, ExprFalseLiteral, ExprFloatLiteral,
    ExprIntDivision, ExprIntLiteral, ExprIntRoot, ExprModulo, ExprMultiplication, ExprNegation,
    ExprPower, ExprReciprocal, ExprRoot, ExprSquare, ExprSquareRoot, ExprSubtraction, ExprVariable,
};

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

    /// Generalizes this object into the [Expr] enum type.
    fn into_expr(self) -> Expr<S> {
        self.into()
    }
}

/// All expressions that are fully characterized by a single enum.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// Not all values of a given variable length enum must be valid for the expression,
/// but all expressions must be representable by a single value of the variable length enum.
pub(crate) trait EnumExpr<S: Sized>:
    Sealed + TryFrom<VariableLengthEnum> + Into<VariableLengthEnum>
{
    fn variable_length_enum(&self) -> &VariableLengthEnum;
}

/// A trait for all integer expression objects.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between integer expression objects.
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
pub(crate) trait IntExpr<S: Sized>: Sealed + Into<Integer> + TryFrom<Integer> {
    /// Converts this object into an [Integer].
    fn into_integer(self) -> Integer {
        self.into()
    }

    fn integer(&self) -> &Integer;

    fn integer_mut(&mut self) -> &mut Integer;
}

/// A trait for all float expression objects.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between float expression objects.
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
pub(crate) trait FloatExpr<S: Sized>: Sealed + Into<Float> + TryFrom<Float> {
    fn into_float(self) -> Float {
        self.into()
    }

    fn float(&self) -> &Float;

    fn float_mut(&mut self) -> &mut Float;
}

/// A trait for all expression objects that hold no value.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between expression objects that hold no value.
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
pub(crate) trait PureExpr<S: Sized>: Sealed + From<()> {}

/// A trait for all binary operation expression objects.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between expression objects that represent
/// an operation between two expressions.
/// Note, that all expressions can be connected using binary operations.
pub(crate) trait BinaryOperationExpr<S: Sized>:
    Sealed + Into<(S, S)> + From<(S, S)>
{
    fn lhs(&self) -> &S;
    fn rhs(&self) -> &S;

    fn lhs_mut(&mut self) -> &mut S;
    fn rhs_mut(&mut self) -> &mut S;

    fn into_parts(self) -> (S, S) {
        self.into()
    }
}

/// A trait for all unary operation expression objects.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between expression objects that represent
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
pub(crate) trait UnaryOperationExpr<S: Sized>: Sealed + From<S> {
    fn inner(&self) -> &S;
    fn inner_mut(&mut self) -> &mut S;

    fn into_inner(self) -> S;
}

macro_rules! compose_expr {
    ($name:ident, $type:ty) => {
        fn $name(&mut self, expr: $type) -> Result<S, ComposeError<Self::Error>> {
            self.compose_default(expr)
        }
    };
}

pub trait Composer<S: Sized> {
    type Error: std::error::Error;

    #[inline]
    #[allow(unused_variables)]
    fn compose_default<E: ExprObj<S>>(&mut self, expr: E) -> Result<S, ComposeError<Self::Error>> {
        Err(ComposeError::DefaultError(
            DefaultComposeError::ComposeNotImplemented,
        ))
    }

    compose_expr!(compose_variable, ExprVariable<S>);
    compose_expr!(compose_true_literal, ExprTrueLiteral<S>);
    compose_expr!(compose_false_literal, ExprFalseLiteral<S>);
    compose_expr!(compose_float_literal, ExprFloatLiteral<S>);
    compose_expr!(compose_int_literal, ExprIntLiteral<S>);
    compose_expr!(compose_addition, ExprAddition<S>);
    compose_expr!(compose_subtraction, ExprSubtraction<S>);
    compose_expr!(compose_multiplication, ExprMultiplication<S>);
    compose_expr!(compose_division, ExprDivision<S>);
    compose_expr!(compose_int_division, ExprIntDivision<S>);
    compose_expr!(compose_modulo, ExprModulo<S>);
    compose_expr!(compose_power, ExprPower<S>);
    compose_expr!(compose_negation, ExprNegation<S>);
    compose_expr!(compose_root, ExprRoot<S>);
    compose_expr!(compose_int_root, ExprIntRoot<S>);
    compose_expr!(compose_square, ExprSquare<S>);
    compose_expr!(compose_cube, ExprCube<S>);
    compose_expr!(compose_square_root, ExprSquareRoot<S>);
    compose_expr!(compose_cube_root, ExprCubeRoot<S>);
    compose_expr!(compose_reciprocal, ExprReciprocal<S>);
}

pub(crate) trait TryReadFromWithComposer<
    R: ?Sized + Read,
    S: Sized,
    C: ?Sized + Config,
    CP: ?Sized + Composer<S>,
>
{
    fn try_read_with_composer(
        byte_stream: &mut R,
        config: &C,
        composer: &mut CP,
    ) -> Result<S, ExprReadWithComposerError<CP::Error>>;
}

pub trait DecompositionRefContainer<'a, S: Sized> {
    fn inner_as_ref(&self) -> &'a Expr<S>;
}

pub trait Decomposer<S: Sized> {
    type Error: std::error::Error;
    fn decompose_as_ref<'a>(
        &mut self,
        storage_ref: &'a S,
    ) -> Result<impl DecompositionRefContainer<'a, S>, DecomposeError<Self::Error>>;
}

pub trait TryWriteToWithDecomposer<
    W: ?Sized + Write,
    S: Sized,
    C: ?Sized + Config,
    DP: ?Sized + Decomposer<S>,
>
{
    fn try_write_with_decomposer(
        &self,
        writer: &mut W,
        config: &C,
        decomposer: &mut DP,
    ) -> Result<(), ExprWriteWithDecomposerError<DP::Error>>;
}
