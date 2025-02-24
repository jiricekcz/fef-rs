//! Traits for expressions and their parsing/writing.
//!
//! # Sealed Traits
//! Some traits are sealed. That means they are not intended to be implemented outside of this crate. Currently this is achieved by using a private trait bound for the
//! implementation of the trait. This is considered a workaround until sealed traits are stabilized in Rust. If the way sealed traits are implemented in rust will be
//! compatible with our current workaround, a switch will be made to the official way of sealing traits.
//!
//! You can determine, whether a trait is sealed by looking at the trait signature, if it contains the `Sealed` trait.
//!
//! Sealed trait:
//! ```compile_fail
//! # use fef::common::traits::private::Sealed;
//! pub trait MyTrait: Sealed {
//!     // Trait body
//! }
//! ```
//!
//! Non-sealed trait:
//! ```rust
//! pub trait MyTrait {
//!    // Trait body
//! }
//! ```

use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        expr::{
            error::{ComposeError, DefaultComposeError, ExprReadWithComposerError},
            Expr, ExprTrueLiteral,
        },
        raw::VariableLengthEnum,
        tokens::ExprToken,
    },
};

use super::{
    error::{DecomposeError, ExprWriteWithDecomposerError},
    ExprAddition, ExprCube, ExprCubeRoot, ExprDivision, ExprFalseLiteral, ExprFloatLiteral,
    ExprIntDivision, ExprIntRoot, ExprModulo, ExprMultiplication, ExprNegation, ExprPower,
    ExprReciprocal, ExprRoot, ExprSignedIntLiteral, ExprSquare, ExprSquareRoot, ExprSubtraction,
    ExprUnsignedIntLiteral, ExprVariable,
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
/// It is used for all common behavior between expression objects that represent
/// an operation between two expressions and it is used mostly to ensure, that
/// all binary operation expressions have all the necessary methods implemented.
///
/// Note, that all expressions can be connected using binary operations.
///
/// # Type Parameters
///
/// * `S`: The type of the storage of child expressions of this expression.
///
/// # Sealed
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait BinaryOperationExpr<S: Sized>: Sealed + Into<(S, S)> + From<(S, S)> {
    /// Returns a reference to the left-hand side of the binary operation.
    ///
    /// # Examples
    /// Using addition of two [`isize`]s:
    /// ```rust
    /// # use fef::v0::expr::{ExprAddition, traits::BinaryOperationExpr};
    /// let addition = ExprAddition::from((1, 2));
    ///
    /// assert_eq!(addition.lhs(), &1);
    /// ```
    fn lhs(&self) -> &S;

    /// Returns a reference to the right-hand side of the binary operation.
    ///
    /// # Examples
    /// Using addition of two [`isize`]s:
    /// ```rust
    /// # use fef::v0::expr::{ExprAddition, traits::BinaryOperationExpr};
    /// let addition = ExprAddition::from((1, 2));
    ///
    /// assert_eq!(addition.rhs(), &2);
    /// ```
    fn rhs(&self) -> &S;

    /// Returns a mutable reference to the left-hand side of the binary operation.
    ///
    /// # Examples
    /// Using addition of two [`isize`]s:
    /// ```rust
    /// # use fef::v0::expr::{ExprAddition, traits::BinaryOperationExpr};
    /// let mut addition = ExprAddition::from((1, 2));
    ///
    /// *addition.lhs_mut() = 3;
    /// assert_eq!(addition.lhs(), &3);
    /// ```
    fn lhs_mut(&mut self) -> &mut S;

    /// Returns a mutable reference to the right-hand side of the binary operation.
    ///
    /// # Examples
    /// Using addition of two [`isize`]s:
    /// ```rust
    /// # use fef::v0::expr::{ExprAddition, traits::BinaryOperationExpr};
    /// let mut addition = ExprAddition::from((1, 2));
    ///
    /// *addition.rhs_mut() = 3;
    /// assert_eq!(addition.rhs(), &3);
    fn rhs_mut(&mut self) -> &mut S;
}

/// A trait for all unary operation expression objects.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is used for all common behavior between expression objects that represent
/// an operation on a single expression and it is used mostly to ensure, that
/// all unary operation expressions have all the necessary methods implemented.
///
/// Note, that all expressions can be connected using unary operations.
///
/// # Type Parameters
///
/// * `S`: The type of the storage of child expressions of this expression.
///
/// # Sealed
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait UnaryOperationExpr<S: Sized>: Sealed + From<S> + AsRef<S> + AsMut<S> {
    /// Returns a reference to the child expression of the unary operation.
    ///
    /// # Examples
    /// Using negation of an [`isize`]:
    /// ```rust
    /// # use fef::v0::expr::{ExprNegation, traits::UnaryOperationExpr};
    /// let negation = ExprNegation::from(1);
    ///
    /// assert_eq!(negation.inner(), &1);
    /// ```
    fn inner(&self) -> &S {
        self.as_ref()
    }

    /// Returns a mutable reference to the child expression of the unary operation.
    ///
    /// # Examples
    /// Using negation of an [`isize`]:
    /// ```rust
    /// # use fef::v0::expr::{ExprNegation, traits::UnaryOperationExpr};
    /// let mut negation = ExprNegation::from(1);
    ///
    /// *negation.inner_mut() = 2;
    /// assert_eq!(negation.inner(), &2);
    /// ```
    fn inner_mut(&mut self) -> &mut S {
        self.as_mut()
    }

    /// Converts into the child expression of the unary operation.
    ///
    /// # Examples
    /// Using negation of an [`isize`]:
    /// ```rust
    /// # use fef::v0::expr::{ExprNegation, traits::UnaryOperationExpr};
    /// let negation = ExprNegation::from(1);
    /// let inner = negation.into_inner();
    /// assert_eq!(inner, 1);
    /// ```
    fn into_inner(self) -> S;
}

macro_rules! compose_expr {
    ($name:ident, $type:ty) => {
        /// Composes this expression type into the storage type `S`.
        ///
        /// Has default implementation that calls [`compose_default`](Self::compose_default), but can be overridden for specific expression types.
        fn $name(&mut self, expr: $type) -> Result<S, ComposeError<Self::Error>> {
            self.compose_default(expr)
        }
    };
}

/// Object used for composing expressions into their storage type.
///
/// # Type Parameters
/// * `S`: The type of the storage of child expressions of this expression.
///
/// # Usage
/// When parsing expressions, you may want to specify how child expressions are composed into their storage type (and also specify the type `S` itself).
/// The implementation of the compositing logic is injected into the parsing process by passing an object that implements this trait.
///
/// It is important to note, that if you are okay with the default [`ExprTree`](crate::v0::expr::ExprTree) type, you do not need to worry about this trait at all.
///
/// ## Implementing a Composer
/// This trait has two types of methods:
/// * [`compose_default`](Composer::compose_default) - This method is called when no specific method is implemented for a given expression type.
/// * `compose_[expr]` - These methods are called when a specific method is implemented for a given expression type.
///
/// If you want to treat all expression types the same, you can implement `compose_default` and default implementations of all compose methods will call this method.
///
/// If you want to treat some expression types differently (e.g. direct evaluation), you can implement the specific compose method for that expression type.
///
/// # Composition Error
/// It is expected, that some composition strategies may be fallible. You can specify the error type using the associated type `Error`.
///
/// # Backwards Compatibility and Breaking Changes
/// To ensure backwards compatibility when adding new expression types, all compose methods have a default implementation that calls `compose_default`.
///
/// # Examples
/// Composer for an `ExprTreeRc`, which is a reference counted version of the [`ExprTree`](crate::v0::expr::ExprTree):
/// ```rust
/// # use std::rc::Rc;
/// # use fef::v0::expr::{Expr, traits::{Composer, ExprObj}, error::ComposeError};
/// struct ExprTreeRc {
///     inner: Rc<Expr<ExprTreeRc>>,
/// }
///
/// struct ExprTreeRcComposer {}
/// impl Composer<ExprTreeRc> for ExprTreeRcComposer {
///    type Error = std::convert::Infallible;
///     
///     fn compose_default<E: ExprObj<ExprTreeRc>>(
///         &mut self, expr: E
///     ) -> Result<ExprTreeRc, ComposeError<Self::Error>> {
///        Ok(ExprTreeRc {
///           inner: Rc::new(expr.into_expr()),
///       })
///    }
/// }
/// ```
///
/// # Data Passing
///
/// You might be asking why parsing methods take a reference to the composer object, not just a generic type parameter.
/// This allows you to save some data in the composer object and use it in the parsing process. This data can even be mutated (all compose functions take a mutable reference to the composer object).
pub trait Composer<S: Sized> {
    type Error: std::error::Error;

    /// Composes the given expression into the storage type `S`.
    ///
    /// This method has by default no information about the expression type. It is better to implement the specific compose methods for each expression type,
    /// if you want to treat them differently. If, however, it is unavoidable, you can call the `Into::<Expr<S>>::into()` method on the generic expression object
    /// to convert it into the [`Expr<S>`] enum and then use pattern matching to determine the expression type. This will however be slower and less maintainable, than
    /// using the specific compose methods.
    ///
    /// Default implementation of this method just returns `Err`.
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
    compose_expr!(compose_signed_int_literal, ExprSignedIntLiteral<S>);
    compose_expr!(compose_unsigned_int_literal, ExprUnsignedIntLiteral<S>);
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

/// Container for a reference to an expression.
///
/// For more information, see [`Decomposer`]. This is chosen over
/// [`AsRef`] because it allows for lifetime to be specified in the trait.
pub trait DecompositionRefContainer<'a, S: Sized> {
    /// Returns a reference to the decomposed expression.
    fn inner_as_ref(&self) -> &'a Expr<S>;
}

impl<'a, S: Sized> DecompositionRefContainer<'a, S> for &'a Expr<S> {
    fn inner_as_ref(&self) -> &'a super::Expr<S> {
        self
    }
}

/// A trait for decomposing a storage type into an expression.
///
/// This trait is the inversion of the [`Composer`] trait. It has a simpler
/// signature, as it only needs to decompose the storage type into an expression and cannot benefit from additional
/// information about the expression type.
pub trait Decomposer<S: Sized> {
    type Error: std::error::Error;
    /// Decomposes the storage type into an expression.
    ///
    /// This method is expected to be fallible, as the storage type may not always be representable as an expression
    /// this is why the error type is specified.
    ///
    /// The `decompose_as_ref` method takes an immutable reference to the storage type (so that it can be used after the decomposition).
    /// This can be useful, as you can, for example, save an expression to a file, but still keep it in memory.
    ///
    /// # Return Type
    /// In most applications, it will be simple to retrieve a `&'a Expr<S>` from `&'a S` (e.g. if `S` is `Box<Expr<S>>`).
    /// In some cases, however, the reference cannot be so easily obtained and may require to be calculated by the decompose method.
    /// This can produce additional data, that `&'a Expr<S>` cannot hold. This is why the return type is an `impl` trait, that implements
    /// a method to get the reference to the decomposed expression. The returned object can have additional data or, for example, a [`Drop`] implementation.
    ///
    /// Note that `&'a Expr<S>` implements the required trait, so you can return a reference to the decomposed expression directly.
    fn decompose_as_ref<'a>(
        &mut self,
        storage_ref: &'a S,
    ) -> Result<impl DecompositionRefContainer<'a, S>, DecomposeError<Self::Error>>;
}
pub(crate) trait TryWriteToWithDecomposer<
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
