use super::Config;

/// Configuration object with default values.
///
/// It is useful for example to parse the configuration of a given file.
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
    /// Creates a new instance of the default configuration. Equivalent to using the [Default] trait.
    pub const fn new() -> Self {
        DefaultConfig {}
    }
}

pub const DEFAULT_CONFIG: DefaultConfig = DefaultConfig::new();
