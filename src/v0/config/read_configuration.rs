use std::io::Read;

use crate::{
    common::stream_utils::skip_bytes,
    v0::{raw::VariableLengthEnum, tokens::ConfigToken, traits::ReadFrom},
};

use super::{default::DEFAULT_CONFIG, error::ConfigurationReadError, Config, OverridableConfig};

impl<R: ?Sized + Read> ReadFrom<R> for OverridableConfig {
    type ReadError = ConfigurationReadError;
    fn read_from<C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let config_count: usize =
            VariableLengthEnum::read_from(reader, configuration)?.try_into()?;

        let mut remaining = config_count;
        let mut output = OverridableConfig::default();

        while remaining > 0 {
            // Looping through the number of configurations given by the first VariableLengthEnum (config_count)
            read_one_config(reader, configuration, &mut output)?;
            remaining -= 1;
        }

        Ok(output)
    }
}

fn skip_non_enum_configuration<R: Read + ?Sized>(
    reader: &mut R,
) -> Result<(), ConfigurationReadError> {
    let byte_length: usize = VariableLengthEnum::read_from(reader, &DEFAULT_CONFIG)?.try_into()?; // Read the byte length of the configuration as a second VariableLengthEnum as per the spec
    skip_bytes(reader, byte_length)?; // Just skip those bytes, we can't do anything with them
    Ok(())
}

fn read_one_config<R: Read + ?Sized, C: ?Sized + Config>(
    reader: &mut R,
    configuration: &C,
    output: &mut OverridableConfig,
) -> Result<(), ConfigurationReadError> {
    let config_token_identifier = VariableLengthEnum::read_from(reader, configuration)?;

    let config_token_identifier_usize =
        match config_token_identifier_to_usize(config_token_identifier, reader)? {
            Some(value) => value,
            None => return Ok(()),
        };

    let config_token = match match_config_token_identifier(
        config_token_identifier_usize,
        reader,
        configuration,
    )? {
        Some(value) => value,
        None => return Ok(()),
    };

    read_enum_configuration(reader, configuration, config_token, output)?;
    Ok(())
}

fn config_token_identifier_to_usize<R: Read + ?Sized>(
    config_token_identifier: VariableLengthEnum,
    reader: &mut R,
) -> Result<Option<usize>, ConfigurationReadError> {
    let config_token_identifier_usize: usize = match config_token_identifier.try_into() {
        Ok(value) => value,
        Err(_) => {
            // Cast to usize failed, identifier is way too large (definitely > 0x7F), so this is a non-enum configuration
            skip_non_enum_configuration(reader)?;
            return Ok(None);
        }
    };
    Ok(Some(config_token_identifier_usize))
}

fn match_config_token_identifier<R: ?Sized + Read, C: ?Sized + Config>(
    config_token_identifier: usize,
    reader: &mut R,
    configuration: &C,
) -> Result<Option<ConfigToken>, ConfigurationReadError> {
    let config_token: ConfigToken = match config_token_identifier.try_into() {
        Ok(token) => token,
        Err(_) => {
            // Identifier is not recognized we decide how to skip it
            if config_token_identifier <= 0x7F {
                // Enum configuration
                let _ = VariableLengthEnum::read_from(reader, configuration)?;
            // Skip one additional VariableLengthEnum
            } else {
                skip_non_enum_configuration(reader)?;
            }
            return Ok(None);
        }
    };
    Ok(Some(config_token))
}

fn read_enum_configuration<R: ?Sized + Read, C: ?Sized + Config>(
    _reader: &mut R,
    _configuration: &C,
    config_token: ConfigToken,
    _output: &mut OverridableConfig,
) -> Result<(), ConfigurationReadError> {
    match config_token {}
}
