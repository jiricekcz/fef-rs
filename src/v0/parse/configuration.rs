use std::io::Read;

use crate::v0::{
    config::{error::ConfigurationReadError, Config, OverridableConfig, DEFAULT_CONFIG},
    traits::ReadFrom,
};

/// Reads a configuration from a byte stream using some configuration.
///
/// This is the most generic method for reading configuration. It assumes
/// that your application has some sort of default configuration that you
/// want to use instead of the default configuration provided by the standard.
///
/// Other than that, it functions the same as [`parse_configuration_with_default_configuration`].
///
/// # Example
/// ```rust
/// # use fef::v0::parse::parse_configuration;
/// # use fef::v0::config::Config;
/// # use fef::v0::config::IntFormat;
/// # use fef::v0::config::FloatFormat;
/// # use fef::v0::config::OverridableConfig;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let bytes: Vec<u8> = vec![
///     0x02, // 2 configurations
///     0x01, 0x01, // Integer format: `i16`,
///     0x00, 0x01, // Float format: `f32`,
/// ];
///
/// let config = OverridableConfig::default();
/// // Edit the configuration to your liking
///
/// let mut reader = &mut bytes.as_slice();
/// let config = parse_configuration(&mut reader, &config)?;
///
/// assert_eq!(config.integer_format(), IntFormat::I16);
/// assert_eq!(config.float_format(), FloatFormat::F32);
/// # assert!(reader.is_empty());
/// # Ok(())
/// # }
pub fn parse_configuration<R: ?Sized + Read, C: ?Sized + Config>(
    byte_stream: &mut R,
    configuration: &C,
) -> Result<OverridableConfig, ConfigurationReadError> {
    OverridableConfig::read_from(byte_stream, configuration)
}

/// Reads a configuration from a byte stream using the default configuration.
///
/// This is the most common way of reading configurations of files you know
/// nothing about. It uses the default configuration provided by the standard.
///
/// # Example
/// ```rust
/// # use fef::v0::parse::parse_configuration_with_default_configuration;
/// # use fef::v0::config::Config;
/// # use fef::v0::config::IntFormat;
/// # use fef::v0::config::FloatFormat;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let bytes: Vec<u8> = vec![
///     0x02, // 2 configurations
///     0x01, 0x01, // Integer format: `i16`,
///     0x00, 0x01, // Float format: `f32`,
/// ];
///
/// let mut reader = &mut bytes.as_slice();
/// let config = parse_configuration_with_default_configuration(&mut reader)?;
///
/// assert_eq!(config.integer_format(), IntFormat::I16);
/// assert_eq!(config.float_format(), FloatFormat::F32);
/// # assert!(reader.is_empty());
/// # Ok(())
/// # }
pub fn parse_configuration_with_default_configuration<R: ?Sized + Read>(
    byte_stream: &mut R,
) -> Result<OverridableConfig, ConfigurationReadError> {
    parse_configuration(byte_stream, &DEFAULT_CONFIG)
}
