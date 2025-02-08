//! Parsing of a fef byte stream
mod configuration;
mod expression;

pub use expression::parse_expression;
pub use expression::parse_expression_into_tree;

pub use configuration::parse_configuration;
