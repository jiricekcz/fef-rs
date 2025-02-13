use crate::{
    common::traits::private::Sealed,
    v0::{
        config::{Config, OverridableConfig},
        expr::ExprTree,
        metadata::MetadataRecord,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct SingleFormulaFile {
    pub(crate) expression: ExprTree,
    pub(crate) configuration: OverridableConfig,
    pub(crate) metadata: Vec<MetadataRecord>,
}

impl SingleFormulaFile {
    pub fn root_expression(&self) -> &ExprTree {
        &self.expression
    }

    pub fn configuration(&self) -> &impl Config {
        &self.configuration
    }

    pub fn metadata_iter(&self) -> impl Iterator<Item = &MetadataRecord> {
        self.metadata.iter()
    }

    pub fn decompose(self) -> (impl Config, Vec<MetadataRecord>, ExprTree) {
        (self.configuration, self.metadata, self.expression)
    }
}

impl Sealed for SingleFormulaFile {}
