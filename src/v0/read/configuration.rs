use std::io::Read;

use crate::v0::{
    config::{error::ConfigurationReadError, Config, OverridableConfig, DEFAULT_CONFIG},
    traits::ReadFrom,
};

/// Reads a [configuration](https://github.com/jiricekcz/fef-specification/blob/main/configuration/Configuration.md) from a byte stream using some configuration.
///
/// This is the most generic method for reading configuration. It assumes
/// that your application has some sort of default configuration that you
/// want to use instead of the default configuration provided by the standard.
///
/// Other than that, it functions the same as [`read_configuration_with_default_configuration`].
///
/// # Example
/// ```rust
/// # use fef::v0::read::read_configuration;
/// # use fef::v0::config::Config;
/// # use fef::v0::config::OverridableConfig;
/// # use fef::v0::config::DEFAULT_CONFIG;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let bytes: Vec<u8> = vec![
///     0x00, // 0 configurations
/// ];
///
/// let mut reader = &mut bytes.as_slice();
/// let config = read_configuration(&mut reader, &DEFAULT_CONFIG)?;
///
/// # assert!(reader.is_empty());
/// # Ok(())
/// # }
pub fn read_configuration<R: ?Sized + Read, C: ?Sized + Config>(
    byte_stream: &mut R,
    configuration: &C,
) -> Result<OverridableConfig, ConfigurationReadError> {
    OverridableConfig::read_from(byte_stream, configuration)
}

/// Reads a [configuration](https://github.com/jiricekcz/fef-specification/blob/main/configuration/Configuration.md) from a byte stream using the default configuration.
///
/// This is the most common way of reading configurations of files you know
/// nothing about. It uses the default configuration provided by the standard.
///
/// # Example
/// ```rust
/// # use fef::v0::read::read_configuration_with_default_configuration;
/// # use fef::v0::config::Config;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let bytes: Vec<u8> = vec![
///     0x00, // 0 configurations
/// ];
///
/// let mut reader = &mut bytes.as_slice();
/// let config = read_configuration_with_default_configuration(&mut reader)?;
/// # assert!(reader.is_empty());
/// # Ok(())
/// # }
pub fn read_configuration_with_default_configuration<R: ?Sized + Read>(
    byte_stream: &mut R,
) -> Result<OverridableConfig, ConfigurationReadError> {
    read_configuration(byte_stream, &DEFAULT_CONFIG)
}
