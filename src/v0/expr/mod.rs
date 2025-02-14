//! Expression part of the [FEF standard](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Expression.md).
//!
//! FEF stores the formula as an expression tree in prefix notation.
//! This library provides great modularity when it comes to storing expressions in memory using the [`Expr`] enum.
//! Working with the [`Expr`] enum directly can be however unnecessarily verbose, if you don't need full control over the storage of child expressions.
//! That's why this library provides the [`ExprTree`] wrapper, which represents children as `Box<ExprTree>` - the most intuitive way to store a tree structure in memory.
//! Unless you have a special use case, [`ExprTree`] is probably the type you want to use.

mod expr;
mod exprs;
mod read_from;
mod write_to;

pub mod error;
pub mod traits;

pub use expr::Expr;
pub use expr::ExprTree;
pub use exprs::*;
