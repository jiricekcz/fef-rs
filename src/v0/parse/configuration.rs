use std::io::Read;

use crate::v0::{
    config::{error::ConfigurationReadError, ReadConfigurationOutput},
    traits::ReadFromWithDefaultConfig,
};

pub fn parse_configuration<R: ?Sized + Read>(
    byte_stream: &mut R,
) -> Result<ReadConfigurationOutput, ConfigurationReadError> {
    ReadConfigurationOutput::read_from(byte_stream)
}
