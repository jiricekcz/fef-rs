use std::io::Read;

use crate::{
    common::{stream_utils::skip_bytes, traits::private::Sealed},
    v0::{
        raw::VariableLengthEnum,
        tokens::ConfigToken,
        traits::{ReadFrom, ReadFromWithDefaultConfig},
    },
};

use super::{
    default::DEFAULT_CONFIG, error::ConfigurationReadError, Config, FloatFormat, IntFormat,
};
pub struct ReadConfigurationOutput {
    pub(crate) integer_format: Option<IntFormat>,
    pub(crate) float_format: Option<FloatFormat>,
}

pub enum ConfigurationValue<T: Default + Copy> {
    Unset,
    Set(T),
}

impl<T: Default + Copy> From<Option<T>> for ConfigurationValue<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => ConfigurationValue::Set(value),
            None => ConfigurationValue::Unset,
        }
    }
}
impl<T: Default + Copy> From<ConfigurationValue<T>> for Option<T> {
    fn from(value: ConfigurationValue<T>) -> Self {
        match value {
            ConfigurationValue::Set(value) => Some(value),
            ConfigurationValue::Unset => None,
        }
    }
}
impl<T: Default + Copy> Default for ConfigurationValue<T> {
    fn default() -> Self {
        ConfigurationValue::Set(T::default())
    }
}

impl<T: Default + Copy> ConfigurationValue<T> {
    pub fn is_set(&self) -> bool {
        matches!(self, ConfigurationValue::Set(_))
    }
    pub fn is_unset(&self) -> bool {
        !self.is_set()
    }
    pub fn into_value(self) -> T {
        match self {
            ConfigurationValue::Set(value) => value,
            ConfigurationValue::Unset => T::default(),
        }
    }
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            ConfigurationValue::Set(value) => Some(value),
            ConfigurationValue::Unset => None,
        }
    }
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            ConfigurationValue::Set(value) => Some(value),
            ConfigurationValue::Unset => None,
        }
    }
    pub fn set(&mut self, value: T) {
        *self = ConfigurationValue::Set(value);
    }
    pub fn set_default(&mut self) {
        *self = ConfigurationValue::Set(T::default());
    }
    pub fn unset(&mut self) {
        *self = ConfigurationValue::Unset;
    }
}

impl ReadConfigurationOutput {
    pub fn integer_format(&self) -> ConfigurationValue<IntFormat> {
        self.integer_format.into()
    }

    pub fn float_format(&self) -> ConfigurationValue<FloatFormat> {
        self.float_format.into()
    }
}

impl Default for ReadConfigurationOutput {
    fn default() -> Self {
        ReadConfigurationOutput {
            integer_format: None,
            float_format: None,
        }
    }
}
impl Sealed for ReadConfigurationOutput {}

impl<R: ?Sized + Read> ReadFromWithDefaultConfig<R> for ReadConfigurationOutput {
    type ReadError = ConfigurationReadError;
    fn read_from(reader: &mut R) -> Result<Self, Self::ReadError> {
        let config_count: usize =
            VariableLengthEnum::read_from(reader, &DEFAULT_CONFIG)?.try_into()?;

        let mut remaining = config_count;
        let mut output = ReadConfigurationOutput::default();

        while remaining > 0 {
            // Looping through the number of configurations given by the first VariableLengthEnum (config_count)
            read_one_config(reader, &mut output)?;
            remaining -= 1;
        }

        Ok(output)
    }
}

impl Config for ReadConfigurationOutput {
    fn float_format(&self) -> FloatFormat {
        self.float_format().into_value()
    }
    fn integer_format(&self) -> IntFormat {
        self.integer_format().into_value()
    }
}

macro_rules! read_enum_configuration {
    ($configuration_type:ty, $reader:ident) => {{
        let variable_length_enum: VariableLengthEnum =
            VariableLengthEnum::read_from($reader, &DEFAULT_CONFIG)?;
        let int_format = <$configuration_type>::try_from(variable_length_enum)?;
        int_format
    }};
}

fn skip_non_enum_configuration<R: Read + ?Sized>(
    reader: &mut R,
) -> Result<(), ConfigurationReadError> {
    let byte_length: usize = VariableLengthEnum::read_from(reader, &DEFAULT_CONFIG)?.try_into()?; // Read the byte length of the configuration as a second VariableLengthEnum as per the spec
    skip_bytes(reader, byte_length)?; // Just skip those bytes, we can't do anything with them
    Ok(())
}

fn read_one_config<R: Read + ?Sized>(
    reader: &mut R,
    output: &mut ReadConfigurationOutput,
) -> Result<(), ConfigurationReadError> {
    let config_token_identifier = VariableLengthEnum::read_from(reader, &DEFAULT_CONFIG)?;

    let config_token_identifier_usize =
        match config_token_identifier_to_usize(config_token_identifier, reader)? {
            Some(value) => value,
            None => return Ok(()),
        };

    let config_token = match match_config_token_identifier(config_token_identifier_usize, reader)? {
        Some(value) => value,
        None => return Ok(()),
    };

    read_enum_configuration(reader, config_token, output)?;
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

fn match_config_token_identifier<R: ?Sized + Read>(
    config_token_identifier: usize,
    reader: &mut R,
) -> Result<Option<ConfigToken>, ConfigurationReadError> {
    let config_token: ConfigToken = match config_token_identifier.try_into() {
        Ok(token) => token,
        Err(_) => {
            // Identifier is not recognized we decide how to skip it
            if config_token_identifier <= 0x7F {
                // Enum configuration
                let _ = VariableLengthEnum::read_from(reader, &DEFAULT_CONFIG)?;
            // Skip one additional VariableLengthEnum
            } else {
                skip_non_enum_configuration(reader)?;
            }
            return Ok(None);
        }
    };
    Ok(Some(config_token))
}

fn read_enum_configuration<R: ?Sized + Read>(
    reader: &mut R,
    config_token: ConfigToken,
    output: &mut ReadConfigurationOutput,
) -> Result<(), ConfigurationReadError> {
    match config_token {
        ConfigToken::IntFormat => {
            let int_format = read_enum_configuration!(IntFormat, reader);
            output.integer_format = Some(int_format);
        }
        ConfigToken::FloatFormat => {
            let float_format = read_enum_configuration!(FloatFormat, reader);
            output.float_format = Some(float_format);
        }
    }
    Ok(())
}
