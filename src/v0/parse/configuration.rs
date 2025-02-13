use std::io::Read;

use crate::v0::{
    config::{error::ConfigurationReadError, Config, ReadConfigurationOutput, DEFAULT_CONFIG},
    traits::ReadFrom,
};

pub fn parse_configuration<R: ?Sized + Read, C: ?Sized + Config>(
    byte_stream: &mut R,
    configuration: &C,
) -> Result<ReadConfigurationOutput, ConfigurationReadError> {
    ReadConfigurationOutput::read_from(byte_stream, configuration)
}

pub fn parse_configuration_with_default_configuration<R: ?Sized + Read>(
    byte_stream: &mut R,
) -> Result<ReadConfigurationOutput, ConfigurationReadError> {
    parse_configuration(byte_stream, &DEFAULT_CONFIG)
}
