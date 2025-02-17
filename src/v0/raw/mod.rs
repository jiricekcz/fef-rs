//! Data-types representing [raw FEF values](https://github.com/jiricekcz/fef-specification/blob/main/binary_types/Binary%20Type.md).

mod variable_length_enum;
pub use variable_length_enum::*;

mod integer;
pub use integer::*;

mod float;
pub use float::*;

pub mod error;

mod string;
