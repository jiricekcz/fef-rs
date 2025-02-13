use crate::common::traits::private::Sealed;

use super::{configurations::*, Config};

/// A configuration starting with values from the default configuration.
///
/// This is the most common configuration structure. It can hold all configurations, but also keeps track, of which values are default.
/// It is possible to differentiate between value being not set and being set to the same value as the default.
///
/// # Example
/// ```rust
/// # use fef::v0::config::{DefaultConfig, OverridableConfig, IntFormat, Config};
/// let mut config = OverridableConfig::default();
///     // Create a new configuration with no values overridden
///
/// assert_eq!(config.integer_format(), IntFormat::default());
///     // Value is correctly default
///
/// assert!(!config.is_integer_format_overridden());
///
/// config.override_integer_format(IntFormat::default());
///     // Override the integer format with its default value
///
/// assert_eq!(config.integer_format(), IntFormat::default());
///     // Value didn't change, since we overrode it with the same value
/// assert!(config.is_integer_format_overridden());
///     // Value was overridden, even though it is the same as the default
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OverridableConfig {
    pub(crate) integer_format: Option<IntFormat>,
    pub(crate) float_format: Option<FloatFormat>,
}

impl Sealed for OverridableConfig {}

impl Config for OverridableConfig {
    /// If the integer format is overridden, returns the overridden value, otherwise the default value.
    fn integer_format(&self) -> IntFormat {
        self.integer_format.unwrap_or_default()
    }

    /// If the float format is overridden, returns the overridden value, otherwise the default value.
    fn float_format(&self) -> FloatFormat {
        self.float_format.unwrap_or_default()
    }
}

impl OverridableConfig {
    /// Overrides the integer format with the given value.
    pub fn override_integer_format(&mut self, format: IntFormat) {
        self.integer_format = Some(format);
    }

    /// Returns whether the integer format is overridden.
    pub fn is_integer_format_overridden(&self) -> bool {
        self.integer_format.is_some()
    }

    /// Overrides the float format with the given value.
    pub fn override_float_format(&mut self, format: FloatFormat) {
        self.float_format = Some(format);
    }

    /// Returns whether the float format is overridden.
    pub fn is_float_format_overridden(&self) -> bool {
        self.float_format.is_some()
    }

    /// Overrides the configuration with another OverridableConfig.
    /// If the other configuration has a value set, it will override the value in this configuration.
    /// If the other configuration does not have a value set, the value in this configuration will remain unchanged.
    ///
    /// # Example
    /// ```rust
    /// # use fef::v0::config::{DefaultConfig, OverridableConfig, FloatFormat, IntFormat, Config};
    /// let mut config = OverridableConfig::default();
    ///
    /// let mut other = OverridableConfig::default();
    /// other.override_integer_format(IntFormat::U8);
    ///
    /// config.override_with(&other);
    ///
    /// assert_eq!(config.integer_format(), IntFormat::U8);
    /// assert!(!config.is_float_format_overridden());
    ///
    /// other.override_float_format(FloatFormat::default());
    /// config.override_with(&other);
    ///
    /// assert_eq!(config.integer_format(), IntFormat::U8);
    /// assert_eq!(config.float_format(), FloatFormat::default());
    ///
    /// assert!(config.is_integer_format_overridden());
    /// assert!(config.is_float_format_overridden());
    ///     // The float format was overridden, even though it has default value
    /// ```
    pub fn override_with(&mut self, other: &OverridableConfig) {
        if let Some(format) = other.integer_format {
            self.override_integer_format(format);
        }
        if let Some(format) = other.float_format {
            self.override_float_format(format);
        }
    }

    pub(crate) fn from_config_full_override<C: ?Sized + Config>(config: &C) -> Self {
        OverridableConfig {
            integer_format: Some(config.integer_format()),
            float_format: Some(config.float_format()),
        }
    }
}

impl Default for OverridableConfig {
    /// Creates a new instance of the configuration with no values overridden.
    ///
    /// # Example
    /// ```rust
    /// # use fef::v0::config::{Config, OverridableConfig, DefaultConfig};
    /// let config = OverridableConfig::default();
    ///
    /// assert_eq!(config.integer_format(), DefaultConfig::new().integer_format());
    /// assert_eq!(config.float_format(), DefaultConfig::new().float_format());
    ///
    /// assert!(!config.is_integer_format_overridden());
    /// assert!(!config.is_float_format_overridden());
    /// ```
    fn default() -> Self {
        OverridableConfig {
            integer_format: None,
            float_format: None,
        }
    }
}
