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

pub fn write_configuration<W: ?Sized + Write, C: ?Sized + Config>(
    writer: &mut W,
    configuration: &C,
) -> Result<(), ConfigurationWriteError> {
    write_enum_configuration!(IntFormat, integer_format, writer, configuration);
    write_enum_configuration!(FloatFormat, float_format, writer, configuration);
    Ok(())
}
