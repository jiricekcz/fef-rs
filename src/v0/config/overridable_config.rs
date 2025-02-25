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
