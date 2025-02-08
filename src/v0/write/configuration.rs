use std::io::Write;

use crate::v0::{
    config::{error::ConfigurationWriteError, Config, EnumConfiguration, FloatFormat, IntFormat},
    raw::VariableLengthEnum,
    traits::WriteTo,
};

macro_rules! write_enum_configuration {
    ($configuration_type:ty, $writer:ident, $configuration:ident) => {{
        let key = <$configuration_type>::token();
        let value = $configuration.value_for_token(key);
        key.variable_length_enum()
            .write_to($writer, $configuration)?;
        VariableLengthEnum::from(value).write_to($writer, $configuration)?;
    }};
}

pub fn write_configuration<W: ?Sized + Write, C: ?Sized + Config>(
    writer: &mut W,
    configuration: &C,
) -> Result<(), ConfigurationWriteError> {
    write_enum_configuration!(IntFormat, writer, configuration);
    write_enum_configuration!(FloatFormat, writer, configuration);
    Ok(())
}
