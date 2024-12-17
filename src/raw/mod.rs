//! Data-types representing raw FEF values.

mod variable_length_enum;

pub use variable_length_enum::*;

mod integer;

pub use integer::*;

pub mod error;

pub mod bytes;
