//! Writing [FEF](https://github.com/jiricekcz/fef-specification/blob/main/README.md) structures into byte streams.
//!
//! Collection of functions that write FEF structures into byte streams. In almost every case, these will be the main
//! functions you will use to interact with this library. Often basic and more advanced functions are provided for
//! convenience and flexibility.
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
