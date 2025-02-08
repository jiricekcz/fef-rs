use crate::common::traits::private::Sealed;

use super::{configurations::*, Config};

/// A configuration starting with values from the default configuration.
pub struct OverridableConfig {
    integer_format: Option<IntFormat>,
    float_format: Option<FloatFormat>,
}

impl Sealed for OverridableConfig {}

impl Config for OverridableConfig {
    fn integer_format(&self) -> IntFormat {
        self.integer_format.unwrap_or_default()
    }

    fn float_format(&self) -> FloatFormat {
        self.float_format.unwrap_or_default()
    }
}

impl OverridableConfig {
    pub fn override_integer_format(&mut self, format: IntFormat) {
        self.integer_format = Some(format);
    }

    pub fn is_integer_format_overridden(&self) -> bool {
        self.integer_format.is_some()
    }

    pub fn override_float_format(&mut self, format: FloatFormat) {
        self.float_format = Some(format);
    }

    pub fn is_float_format_overridden(&self) -> bool {
        self.float_format.is_some()
    }

    pub fn override_with(&mut self, other: OverridableConfig) {
        if let Some(format) = other.integer_format {
            self.override_integer_format(format);
        }
        if let Some(format) = other.float_format {
            self.override_float_format(format);
        }
    }
}

impl Default for OverridableConfig {
    fn default() -> Self {
        OverridableConfig {
            integer_format: None,
            float_format: None,
        }
    }
}
