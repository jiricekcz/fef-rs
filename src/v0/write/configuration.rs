use std::io::Write;

use crate::v0::{
    config::{error::ConfigurationWriteError, Config, EnumConfiguration, FloatFormat, IntFormat},
    raw::VariableLengthEnum,
    traits::WriteTo,
};

macro_rules! write_enum_configuration {
    ($configuration_type:ty, $config_method:ident, $writer:ident, $configuration:ident) => {{
        let key = <$configuration_type>::token();
        let value = $configuration.$config_method().value();
        key.variable_length_enum()
            .write_to($writer, $configuration)?;
        VariableLengthEnum::from(value).write_to($writer, $configuration)?;
    }};
}

/// Writes a [configuration](https://github.com/jiricekcz/fef-specification/blob/main/configuration/Configuration.md) to a byte stream.
///
/// Writes any [`Config`] to byte stream by writing all values.
///
/// # Example
///
/// ```rust
/// # use fef::v0::config::{Config, IntFormat, FloatFormat, OverridableConfig};
/// # use fef::v0::write::write_configuration;
/// # use fef::v0::read::read_configuration_with_default_configuration;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut configuration = OverridableConfig::default();
///
/// configuration.override_integer_format(IntFormat::I8);
/// configuration.override_float_format(FloatFormat::F32);
///
/// let mut writer = Vec::new();
///
/// write_configuration(&mut writer, &configuration)?;
///
/// let reader = &mut writer.as_slice();
/// let read_configuration = read_configuration_with_default_configuration(reader)?;
///
/// assert_eq!(read_configuration.integer_format(), configuration.integer_format());
/// assert_eq!(read_configuration.float_format(), configuration.float_format());
///
/// # Ok(())
/// # }
pub fn write_configuration<W: ?Sized + Write, C: ?Sized + Config>(
    writer: &mut W,
    configuration: &C,
) -> Result<(), ConfigurationWriteError> {
    let record_count = VariableLengthEnum::from(2);

    record_count.write_to(writer, configuration)?;

    write_enum_configuration!(FloatFormat, float_format, writer, configuration);
    write_enum_configuration!(IntFormat, integer_format, writer, configuration);
    Ok(())
}
