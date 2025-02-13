use std::io::Read;

use crate::v0::{
    config::{error::ConfigurationReadError, Config, ReadConfigurationOutput},
    traits::ReadFrom,
};

pub fn parse_configuration<R: ?Sized + Read, C: ?Sized + Config>(
    byte_stream: &mut R,
    configuration: &C,
) -> Result<ReadConfigurationOutput, ConfigurationReadError> {
    ReadConfigurationOutput::read_from(byte_stream, configuration)
}
