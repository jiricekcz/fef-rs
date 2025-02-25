//! Configuration of the FEF parser based on the [specification](https://github.com/jiricekcz/fef-specification/blob/main/configuration/Configuration.md)

mod configurations;
mod default;
pub mod error;
mod overridable_config;
mod read_configuration;
mod traits;

pub use default::DefaultConfig;
pub use default::DEFAULT_CONFIG;
pub use overridable_config::OverridableConfig;
pub use traits::Config;
