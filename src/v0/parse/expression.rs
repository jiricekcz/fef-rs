use std::io::Read;

use crate::v0::{
    config::Config,
    expr::{
        error::{ComposeError, ExprReadWithComposerError},
        traits::{Composer, ExprObj, TryReadFromWithComposer},
        Expr, ExprTree,
    },
};
/// Parses an [expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Expression.md) from a byte stream using a composer.
///
/// Reads an expression in the FEF (prefix) format from the byte stream and composes it into a custom type using the [composer](crate::v0::expr::traits::Composer).
/// Whenever an expression is parsed in the proces, the appropriate method of the composer is used to convert this expression into `S`. This value is then used
/// to parse parent expressions. The most common type for `S` is probably `Box<Expr<S>>` (an in memory tree) - yielding a recursive type `Expr<Box<Expr<Box<Expr<...>>>>>`.
/// Since it is not possible to express this type directly in Rust, the [`ExprTree`] type is provided, which provides this functionality. If you want to parse to
/// [`ExprTree`], use the [`parse_expression_into_tree`] function.
///
/// # Type parameters
///
/// - `R`: The type of the byte stream reader.
/// - `C`: The type of the configuration.
/// - `S`: The type of the expression.
/// - `CP`: The type of the composer.
///
/// All but the `S` type parameter should in most cases be inferred.
///
/// # Usage
///
/// For usage, see the [`Composer`] trait.
pub fn parse_expression<
    R: ?Sized + Read,
    C: ?Sized + Config,
    S: Sized,
    CP: ?Sized + Composer<S>,
>(
    byte_stream: &mut R,
    config: &C,
    composer: &mut CP,
) -> Result<S, ExprReadWithComposerError<CP::Error>> {
    <Expr<S> as TryReadFromWithComposer<R, S, C, CP>>::try_read_with_composer(
        byte_stream,
        config,
        composer,
    )
}

/// Parses an [expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Expression.md) from a byte stream and returns it as an [`ExprTree`].
///
/// This function is a convenience function that simplifies calling [`parse_expression`] with a composer that composes to an [`ExprTree`].
/// For more information on parsing expressions, see the [`parse_expression`] function.
///
/// # Example
/// Parsing the quadratic formula expression:
/// ```rust
/// # use fef::v0::parse::parse_expression_into_tree;
/// # use fef::v0::config::DEFAULT_CONFIG;
/// # use fef::v0::expr::ExprTree;
/// # use fef::v0::expr::Expr;
/// # use fef::v0::expr::ExprFalseLiteral;
/// # use fef::v0::expr::ExprVariable;
/// # use fef::v0::expr::ExprIntLiteral;
/// # use fef::v0::expr::ExprAddition;
/// # use fef::v0::expr::ExprSubtraction;
/// # use fef::v0::expr::ExprMultiplication;
/// # use fef::v0::expr::ExprDivision;
/// # use fef::v0::expr::ExprSquareRoot;
/// # use fef::v0::expr::ExprSquare;
/// # use fef::v0::expr::ExprNegation;
/// # use fef::v0::raw::VariableLengthEnum;
/// # use fef::v0::raw::Integer;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let bytes: Vec<u8> = vec![
///     0x13, // Divide
///         0x10, // Add - we will use the positive part
///             0x17, // Negation
///                 0x04, 0x01, // Variable 1 (b)
///             0x22, // Square root
///                 0x11, // Subtract
///                    0x20, // Square
///                        0x04, 0x01, // Variable 1 (b)
///                    0x12, // Multiply
///                       0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, // Number 4
///                       0x12, // Multiply
///                           0x04, 0x00, // Variable 0 (a)
///                           0x04, 0x02, // Variable 2 (c)    
///         0x12, // Multiply
///             0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // Number 2
///             0x04, 0x00, // Variable 0 (a)
/// ];
///
/// let a: ExprTree = Expr::<ExprTree>::Variable(VariableLengthEnum::from(0).into()).into();
/// let b: ExprTree = Expr::<ExprTree>::Variable(VariableLengthEnum::from(1).into()).into();
/// let c: ExprTree = Expr::<ExprTree>::Variable(VariableLengthEnum::from(2).into()).into();
///
/// let four: ExprTree = Expr::<ExprTree>::IntLiteral(
///     ExprIntLiteral::from(Integer::from(4))
/// ).into();
/// let two: ExprTree = Expr::<ExprTree>::IntLiteral(
///     ExprIntLiteral::from(Integer::from(2))
/// ).into();
///
/// let ac: ExprTree = Expr::<ExprTree>::Multiplication(
///     ExprMultiplication::from((a.clone(), c.clone()))
/// ).into();
/// let four_ac: ExprTree = Expr::<ExprTree>::Multiplication(
///     ExprMultiplication::from((four.clone(), ac))
/// ).into();
///
/// let b_square: ExprTree = Expr::<ExprTree>::Square(
///     ExprSquare::from(b.clone())
/// ).into();
/// let b_square_minus_four_ac: ExprTree = Expr::<ExprTree>::Subtraction(
///     ExprSubtraction::from((b_square, four_ac))
/// ).into();
///
/// let sqrt_b_square_minus_four_ac: ExprTree = Expr::<ExprTree>::SquareRoot(
///     ExprSquareRoot::from(b_square_minus_four_ac)
/// ).into();
///
/// let minus_b: ExprTree = Expr::<ExprTree>::Negation(
///     ExprNegation::from(b.clone())
/// ).into();
///
/// let numerator: ExprTree = Expr::<ExprTree>::Addition(
///     ExprAddition::from((minus_b, sqrt_b_square_minus_four_ac))
/// ).into();
///
///
/// let denominator: ExprTree = Expr::<ExprTree>::Multiplication(
///     ExprMultiplication::from((two.clone(), a.clone()))
/// ).into();
///
///
/// let fraction: ExprTree = Expr::<ExprTree>::Division(
///     ExprDivision::from((numerator, denominator))
/// ).into();
///
///
/// let mut reader = &mut bytes.as_slice();
/// let expr = parse_expression_into_tree(&mut reader, &DEFAULT_CONFIG)?;
///
/// assert_eq!(fraction, expr);
/// # Ok(())
/// # }
pub fn parse_expression_into_tree<R: ?Sized + Read, C: ?Sized + Config>(
    byte_stream: &mut R,
    config: &C,
) -> Result<ExprTree, ExprReadWithComposerError<std::convert::Infallible>> {
    let mut composer = ExprTreeComposer {};
    parse_expression(byte_stream, config, &mut composer)
}

struct ExprTreeComposer {}
impl Composer<ExprTree> for ExprTreeComposer {
    type Error = std::convert::Infallible;
    fn compose_default<E: ExprObj<ExprTree>>(
        &mut self,
        expr: E,
    ) -> Result<ExprTree, ComposeError<Self::Error>> {
        Ok(ExprTree::from(expr.into()))
    }
}
