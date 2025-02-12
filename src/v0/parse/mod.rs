//! Parsing of a fef byte stream
mod configuration;
mod expression;
mod metadata;

pub use expression::parse_expression;
pub use expression::parse_expression_into_tree;

pub use configuration::parse_configuration;

pub use metadata::parse_metadata;
