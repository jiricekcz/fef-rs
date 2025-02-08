use super::Config;

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
