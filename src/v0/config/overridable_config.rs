use crate::common::traits::private::Sealed;

use super::Config;

/// Configuration object with defaults and the ability to override values.
///
/// This is the most common configuration structure. It can hold all configurations, but also keeps track, of which values are default.
/// It is possible to differentiate between value being not set and being set to the same value as the default.
///
/// # Example
/// ```rust
/// # use fef::v0::config:: OverridableConfig;
/// let mut config = OverridableConfig::default();
/// // There are currently no configurations
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OverridableConfig {}

impl Sealed for OverridableConfig {}

impl Config for OverridableConfig {}

impl OverridableConfig {
    /// Overrides the configuration with another OverridableConfig.
    /// If the other configuration has a value set, it will override the value in this configuration.
    /// If the other configuration does not have a value set, the value in this configuration will remain unchanged.
    ///
    /// # Example
    /// ```rust
    /// # use fef::v0::config::{DefaultConfig, OverridableConfig, Config};
    /// let mut config = OverridableConfig::default();
    ///
    /// let mut other = OverridableConfig::default();
    ///
    /// config.override_with(&other);
    /// ```
    pub fn override_with(&mut self, _other: &OverridableConfig) {}

    pub(crate) fn from_config_full_override<C: ?Sized + Config>(_config: &C) -> Self {
        OverridableConfig {}
    }
}

impl Default for OverridableConfig {
    /// Creates a new instance of the configuration with no values overridden.
    ///
    /// # Example
    /// ```rust
    /// # use fef::v0::config::{Config, OverridableConfig, DefaultConfig};
    /// let config = OverridableConfig::default();
    /// // There are currently no configurations
    /// ```
    fn default() -> Self {
        OverridableConfig {}
    }
}
