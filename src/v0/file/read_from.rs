use std::io::Read;

use crate::v0::{
    config::{Config, OverridableConfig},
    metadata::MetadataRecord,
    parse::{parse_configuration, parse_expression_into_tree, parse_metadata_as_vec},
    traits::ReadFrom,
};

use super::{
    error::{RawFormulaReadError, SingleFormulaReadError},
    RawFormulaFile, SingleFormulaFile,
};

/// Reads a single formula file from a byte stream. Expects, that the file content type identifier has already been read.
impl<R: ?Sized + Read> ReadFrom<R> for SingleFormulaFile {
    type ReadError = SingleFormulaReadError;

    /// Reads a single formula file from a byte stream. Expects, that the file content type identifier has already been read.
    ///
    /// It reads the configuration, metadata and expression from the byte stream.
    ///
    /// For more fine-grained control over the reading process, use [`parse_configuration`](crate::v0::parse::parse_configuration), [`parse_metadata`](crate::v0::parse::parse_metadata) and [`parse_expression`](crate::v0::parse::parse_expression) directly.
    /// Note that when this methods returns, all the data has been read from the byte stream, unlike reading metadata with the [`parse_metadata`](crate::v0::parse::parse_metadata) function.
    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, SingleFormulaReadError> {
        let mut config = OverridableConfig::from_config_full_override(configuration);
        let file_config = parse_configuration(reader, configuration)?;
        config.override_with(&file_config.clone().into());

        let metadata: Vec<MetadataRecord> = parse_metadata_as_vec(reader, configuration)?;

        let expression = parse_expression_into_tree(reader, &config)?;

        Ok(SingleFormulaFile {
            configuration: file_config,
            expression,
            metadata,
        })
    }
}

/// Reads a raw formula file from a byte stream. Expects, that the file content type identifier has already been read.
impl<R: ?Sized + Read> ReadFrom<R> for RawFormulaFile {
    type ReadError = RawFormulaReadError;

    /// Reads a raw formula file from a byte stream. Expects, that the file content type identifier has already been read.
    ///
    /// It is essentially a wrapper around [`parse_expression_into_tree`], because a raw formula file only contains an expression.
    ///
    /// For more fine-grained control over the reading process, use [`parse_expression`](crate::v0::parse::parse_expression) directly.
    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let expression = parse_expression_into_tree(reader, configuration)?;
        Ok(RawFormulaFile { expression })
    }
}
