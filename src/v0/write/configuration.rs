use std::io::Write;

use crate::v0::{
    config::{error::ConfigurationWriteError, Config},
    raw::VariableLengthEnum,
    traits::WriteTo,
};

/// Writes a [configuration](https://github.com/jiricekcz/fef-specification/blob/main/configuration/Configuration.md) to a byte stream.
///
/// Writes any [`Config`] to byte stream by writing all values.
///
/// # Example
///
/// ```rust
/// # use fef::v0::config::{Config, OverridableConfig};
/// # use fef::v0::write::write_configuration;
/// # use fef::v0::read::read_configuration_with_default_configuration;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut configuration = OverridableConfig::default();
///
/// let mut writer = Vec::new();
///
/// write_configuration(&mut writer, &configuration)?;
///
/// let reader = &mut writer.as_slice();
/// let read_configuration = read_configuration_with_default_configuration(reader)?;
///
/// # assert!(reader.is_empty());
/// # Ok(())
/// # }
pub fn write_configuration<W: ?Sized + Write, C: ?Sized + Config>(
    writer: &mut W,
    configuration: &C,
) -> Result<(), ConfigurationWriteError> {
    let record_count = VariableLengthEnum::from(0);

    record_count.write_to(writer, configuration)?;

    Ok(())
}
