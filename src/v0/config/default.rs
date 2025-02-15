use super::Config;

/// Configuration object with default values.
///
/// It is useful for example to read the configuration of a given file.
///
/// # Example
/// All of the configuration values are set to their default values.
/// ```rust
/// # use fef::v0::config::{IntFormat, FloatFormat, DefaultConfig, Config};
/// let config = DefaultConfig::new();
///
/// assert_eq!(config.float_format(), FloatFormat::default());
/// assert_eq!(config.integer_format(), IntFormat::default());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultConfig {}
impl Config for DefaultConfig {}

impl Default for DefaultConfig {
    /// Creates a new instance of the default configuration.
    fn default() -> Self {
        DefaultConfig {}
    }
}

impl DefaultConfig {
    /// Creates a new instance of the default configuration. Equivalent to using the [`Default`] trait.
    pub const fn new() -> Self {
        DefaultConfig {}
    }
}

/// Default configuration object.
///
/// This is an instance of the [`DefaultConfig`] struct, which can be used instead of creating a new instance.
pub const DEFAULT_CONFIG: DefaultConfig = DefaultConfig::new();
