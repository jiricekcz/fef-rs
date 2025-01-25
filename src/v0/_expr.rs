//! Expression representation as defined in the FEF specification.

use crate::v0::raw::VariableLengthEnum;

/// Variable expression as defined in the FEF specification.
///
/// Holds only the id of the variable.
#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ExprVariable {
    /// The id of the variable.
    pub id: VariableLengthEnum, // This structure is stable in the spec and thus can be used as is.
}

pub struct ExprIntLiteral {}

pub struct ExprFloatLiteral {}

pub struct ExprTrueLiteral {}

pub struct ExprFalseLiteral {}

pub struct ExprAddition {}

pub struct ExprSubtraction {}

pub struct ExprMultiplication {}

pub struct ExprDivision {}

pub struct ExprIntDivision {}

pub struct ExprModulo {}

pub struct ExprPower {}

pub struct ExprNegation {}

pub struct ExprRoot {}

pub struct ExprIntRoot {}

pub struct ExprSquare {}

pub struct ExprCube {}

pub struct ExprSquareRoot {}

pub struct ExprCubeRoot {}

pub struct ExprReciprocal {}

pub trait UnaryOperator {
    fn expr(&self) -> &Expr;
}

pub trait BinaryOperator {
    fn lhs(&self) -> &Expr;
    fn rhs(&self) -> &Expr;
}
