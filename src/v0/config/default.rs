use super::Config;

/// Configuration object with default values.
///
/// It is useful for example to parse the configuration of a given file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultConfig {}
impl Config for DefaultConfig {}

impl Default for DefaultConfig {
    fn default() -> Self {
        DefaultConfig {}
    }
}

impl DefaultConfig {
    pub const fn new() -> Self {
        DefaultConfig {}
    }
}

pub const DEFAULT_CONFIG: DefaultConfig = DefaultConfig::new();
