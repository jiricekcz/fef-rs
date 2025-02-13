use std::io::Read;

use crate::v0::{
    config::{Config, OverridableConfig},
    metadata::{error::MetadataReadError, MetadataRecord},
    parse::{parse_configuration, parse_expression_into_tree, parse_metadata},
    traits::ReadFrom,
};

use super::{
    error::{RawFormulaReadError, SingleFormulaReadError},
    RawFormulaFile, SingleFormulaFile,
};

impl<R: ?Sized + Read> ReadFrom<R> for SingleFormulaFile {
    type ReadError = SingleFormulaReadError;

    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, SingleFormulaReadError> {
        let mut config = OverridableConfig::from_config_full_override(configuration);
        let file_config = parse_configuration(reader, configuration)?;
        config.override_with(&file_config.clone().into());

        let metadata_iterator =
            parse_metadata(reader, &config).map_err(MetadataReadError::HeaderError)?;

        let mut metadata: Vec<MetadataRecord> = Vec::new();
        for record in metadata_iterator {
            metadata.push(record.map_err(MetadataReadError::RecordError)?);
        }

        let expression = parse_expression_into_tree(reader, &config)?;

        Ok(SingleFormulaFile {
            configuration: file_config,
            expression,
            metadata,
        })
    }
}

impl<R: ?Sized + Read> ReadFrom<R> for RawFormulaFile {
    type ReadError = RawFormulaReadError;
    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let expression = parse_expression_into_tree(reader, configuration)?;
        Ok(RawFormulaFile { expression })
    }
}
