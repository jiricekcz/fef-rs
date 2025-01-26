use crate::{common::traits::private::Sealed, v0::tokens::ExprToken};

use super::{traits::ExprObj, *};

#[non_exhaustive]
pub enum Expr<S: Sized> {
    /// Represents any expression in the FEF specification.
    ///
    /// # Non-exhaustive
    /// To allow for adding of new expression types without breaking existing code, this enum is marked as [non-exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).  
    ///
    /// # Examples
    /// Variable expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprVariable).
    Variable(ExprVariable<S>),

    /// Integer literal expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprIntLiteral).
    IntLiteral(ExprIntLiteral<S>),

    /// Float literal expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprFloatLiteral).
    FloatLiteral(ExprFloatLiteral<S>),

    /// True literal expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprTrueLiteral).
    TrueLiteral(ExprTrueLiteral<S>),

    /// False literal expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprFalseLiteral).
    FalseLiteral(ExprFalseLiteral<S>),

    /// Addition expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprAddition).
    Addition(ExprAddition<S>),

    /// Subtraction expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprSubtraction).
    Subtraction(ExprSubtraction<S>),

    /// Multiplication expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprMultiplication).
    Multiplication(ExprMultiplication<S>),

    /// Division expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprDivision).
    Division(ExprDivision<S>),

    /// Integer division expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprIntDivision).
    IntDivision(ExprIntDivision<S>),

    /// Modulo expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprModulo).
    Modulo(ExprModulo<S>),

    /// Power expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprPower).
    Power(ExprPower<S>),

    /// Negation expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprNegation).
    Negation(ExprNegation<S>),

    /// Root expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprRoot).
    Root(ExprRoot<S>),

    /// Integer root expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprIntRoot).
    IntRoot(ExprIntRoot<S>),

    /// Square expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprSquare).
    Square(ExprSquare<S>),

    /// Cube expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprCube).
    Cube(ExprCube<S>),

    /// Square root expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprSquareRoot).
    SquareRoot(ExprSquareRoot<S>),

    /// Cube root expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprCubeRoot).
    CubeRoot(ExprCubeRoot<S>),

    /// Reciprocal expression as defined in the FEF specification. See more [here](crate::v0::expr::ExprReciprocal).
    Reciprocal(ExprReciprocal<S>),
}

impl<S: Sized> Sealed for Expr<S> {}

impl<S: Sized> ExprObj<S> for Expr<S> {
    fn token(&self) -> ExprToken {
        match self {
            Expr::Variable(inner) => ExprObj::<S>::token(inner),
            Expr::IntLiteral(inner) => ExprObj::<S>::token(inner),
            Expr::FloatLiteral(inner) => ExprObj::<S>::token(inner),
            Expr::TrueLiteral(inner) => ExprObj::<S>::token(inner),
            Expr::FalseLiteral(inner) => ExprObj::<S>::token(inner),
            Expr::Addition(inner) => ExprObj::<S>::token(inner),
            Expr::Subtraction(inner) => ExprObj::<S>::token(inner),
            Expr::Multiplication(inner) => ExprObj::<S>::token(inner),
            Expr::Division(inner) => ExprObj::<S>::token(inner),
            Expr::IntDivision(inner) => ExprObj::<S>::token(inner),
            Expr::Modulo(inner) => ExprObj::<S>::token(inner),
            Expr::Power(inner) => ExprObj::<S>::token(inner),
            Expr::Negation(inner) => ExprObj::<S>::token(inner),
            Expr::Root(inner) => ExprObj::<S>::token(inner),
            Expr::IntRoot(inner) => ExprObj::<S>::token(inner),
            Expr::Square(inner) => ExprObj::<S>::token(inner),
            Expr::Cube(inner) => ExprObj::<S>::token(inner),
            Expr::SquareRoot(inner) => ExprObj::<S>::token(inner),
            Expr::CubeRoot(inner) => ExprObj::<S>::token(inner),
            Expr::Reciprocal(inner) => ExprObj::<S>::token(inner),
        }
    }
}

/// A helper new-type-like struct to allow expression trees to be treated as expressions.
///
/// This struct is equivalent to the infinitely recursive type `Expr<Expr<Expr<Expr<...>>>>`, which is due to current limitations
/// of Rust's type system not possible to express directly. This struct is a direct mapping to this type however and so
/// can be freely converted to and from the `Expr` enum using the `Into` and `From` traits.
///
/// # Examples
/// ```
/// use crate::v0::expr::{Expr, ExprTree, ExprVariable};
/// use crate::v0::raw::VariableLengthEnum;
///
/// // Example of converting an Expr to an ExprTree
/// let expr = Expr::Variable(ExprVariable::new(VariableLengthEnum::from(0)));
/// let expr_tree: ExprTree = expr.into();
///
/// // Example of converting an ExprTree back to an Expr
/// let expr: Expr<ExprTree> = expr_tree.into();
/// ```
pub struct ExprTree {
    inner: Expr<ExprTree>,
}

impl From<Expr<ExprTree>> for ExprTree {
    fn from(expr: Expr<ExprTree>) -> Self {
        Self { inner: expr }
    }
}

impl Into<Expr<ExprTree>> for ExprTree {
    fn into(self) -> Expr<ExprTree> {
        self.inner
    }
}
