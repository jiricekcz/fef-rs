//! Parsing of a fef byte stream
//!
//! FEF as a format is intended to be embedded in other formats (most often that will be the [expression](https://github.com/jiricekcz/fef-specification/blob/main/expressions/Expression.md) part).
//! For that reason this library exposes function for parsing top level FEF structures and gives emphasis on those.
//! Function for reading files are also provided, but they are, for now, meant to serve the most common use cases only
//! and do not provide the same level of control as the functions for parsing top level structures.
//!
//! # Common Interface
//!
//! All parsing is done on a byte stream (`&mut R` where `R: std::io::Read`). When it makes sense, the parsing is also done
//! sequentially, so that the whole byte stream does not need to be loaded into memory at once.
mod configuration;
mod expression;
mod file;
mod metadata;

pub use expression::parse_expression;
pub use expression::parse_expression_into_tree;

pub use configuration::parse_configuration;
pub use configuration::parse_configuration_with_default_configuration;

pub use metadata::parse_metadata;
pub use metadata::parse_metadata_as_vec;

pub use file::read_file;
pub use file::read_file_with_default_config;
