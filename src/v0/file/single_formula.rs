use crate::{
    common::traits::private::Sealed,
    v0::{
        config::{Config, OverridableConfig},
        expr::ExprTree,
        metadata::MetadataRecord,
    },
};

/// Contents of a [single formula file](https://github.com/jiricekcz/fef-specification/blob/main/file_content_types/Single%20Formula.md)
///
/// Contains the expression, configuration and metadata.
/// Expression is represented as an expression tree using the [`ExprTree`] type.
/// Configuration is represented as an overridable configuration using the [`OverridableConfig`] type.
/// Metadata is represented as an iterator over metadata records using the [`MetadataRecord`] type.
#[derive(Debug, Clone, PartialEq)]
pub struct SingleFormulaFile {
    pub(crate) expression: ExprTree,
    pub(crate) configuration: OverridableConfig,
    pub(crate) metadata: Vec<MetadataRecord>,
}

impl SingleFormulaFile {
    /// Returns a borrow of the root of the expression tree
    pub fn root_expression(&self) -> &ExprTree {
        &self.expression
    }

    /// Returns a borrow of the configuration
    pub fn configuration(&self) -> &impl Config {
        &self.configuration
    }

    /// Returns an iterator over the metadata records
    pub fn metadata_iter(&self) -> impl Iterator<Item = &MetadataRecord> {
        self.metadata.iter()
    }

    /// Decompose the file into the configuration, metadata records and the [`ExprTree`] root
    pub fn decompose(self) -> (impl Config, Vec<MetadataRecord>, ExprTree) {
        (self.configuration, self.metadata, self.expression)
    }
}

impl Sealed for SingleFormulaFile {}
