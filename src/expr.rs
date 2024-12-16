//! Expression representation as defined in the FEF specification.

/// Represents any expression in the FEF specification.
///
/// # Non-exhaustive
/// To allow for adding of new expression types without breaking existing code, this enum is marked as [non-exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).  
///
/// # Examples
#[non_exhaustive]
pub enum Expr {
    /// Variable expression as defined in the FEF specification. See more [here](crate::expr::ExprVariable).
    Variable(ExprVariable),

    /// Integer literal expression as defined in the FEF specification. See more [here](crate::expr::ExprIntLiteral).
    IntLiteral(ExprIntLiteral),

    /// Float literal expression as defined in the FEF specification. See more [here](crate::expr::ExprFloatLiteral).
    FloatLiteral(ExprFloatLiteral),

    /// True literal expression as defined in the FEF specification. See more [here](crate::expr::ExprTrueLiteral).
    TrueLiteral(ExprTrueLiteral),

    /// False literal expression as defined in the FEF specification. See more [here](crate::expr::ExprFalseLiteral).
    FalseLiteral(ExprFalseLiteral),

    /// Addition expression as defined in the FEF specification. See more [here](crate::expr::ExprAddition).
    Addition(ExprAddition),

    /// Subtraction expression as defined in the FEF specification. See more [here](crate::expr::ExprSubtraction).
    Subtraction(ExprSubtraction),

    /// Multiplication expression as defined in the FEF specification. See more [here](crate::expr::ExprMultiplication).
    Multiplication(ExprMultiplication),

    /// Division expression as defined in the FEF specification. See more [here](crate::expr::ExprDivision).
    Division(ExprDivision),

    /// Integer division expression as defined in the FEF specification. See more [here](crate::expr::ExprIntDivision).
    IntDivision(ExprIntDivision),

    /// Modulo expression as defined in the FEF specification. See more [here](crate::expr::ExprModulo).
    Modulo(ExprModulo),

    /// Power expression as defined in the FEF specification. See more [here](crate::expr::ExprPower).
    Power(ExprPower),

    /// Negation expression as defined in the FEF specification. See more [here](crate::expr::ExprNegation).
    Negation(ExprNegation),

    /// Root expression as defined in the FEF specification. See more [here](crate::expr::ExprRoot).
    Root(ExprRoot),

    /// Integer root expression as defined in the FEF specification. See more [here](crate::expr::ExprIntRoot).
    IntRoot(ExprIntRoot),

    /// Square expression as defined in the FEF specification. See more [here](crate::expr::ExprSquare).
    Square(ExprSquare),

    /// Cube expression as defined in the FEF specification. See more [here](crate::expr::ExprCube).
    Cube(ExprCube),

    /// Square root expression as defined in the FEF specification. See more [here](crate::expr::ExprSquareRoot).
    SquareRoot(ExprSquareRoot),

    /// Cube root expression as defined in the FEF specification. See more [here](crate::expr::ExprCubeRoot).
    CubeRoot(ExprCubeRoot),

    /// Reciprocal expression as defined in the FEF specification. See more [here](crate::expr::ExprReciprocal).
    Reciprocal(ExprReciprocal),
}

pub struct ExprVariable {}

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
