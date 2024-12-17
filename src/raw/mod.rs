//! Data-types representing raw FEF values.

mod variable_length_enum;
pub use variable_length_enum::*;

mod integer;
pub use integer::*;

mod float;
pub use float::*;

pub mod error;

pub mod bytes;

mod string;
pub use string::*;
