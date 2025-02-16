//! Writing [FEF](https://github.com/jiricekcz/fef-specification/blob/main/README.md) structures into byte streams.
//!
//!
mod configuration;
mod expression;
mod file;
mod metadata;

pub use expression::{write_expression, write_expression_tree};

pub use configuration::write_configuration;

pub use metadata::{write_metadata, write_metadata_from_vec};

pub use file::{
    write_expression_tree_as_raw_formula, write_metadata_vec_expression_tree_as_single_formula,
    write_raw_formula, write_single_formula,
};
