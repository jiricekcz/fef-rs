//! Parsing of a fef byte stream
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
