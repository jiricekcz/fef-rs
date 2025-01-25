//! Configuration of the FEF parser.

/// Configuration requirements for a FEF parser with defaults.
///
/// # Breaking Changes
/// It is not expected, that downstream crates will implement this trait together with other traits that may cause name conflicts.
/// It is thus not considered a breaking change to add new methods to this trait.
/// If you must implement this trait together with other traits, use [disambiguation syntax].
/// ```rust
/// # use fef::v0::config::IntFormat;
/// struct MyConfig;
///
/// impl fef::v0::config::Config for MyConfig {}
///
/// trait MyTrait {
///     fn integer_format(&self) -> IntFormat;
/// }
///
/// impl MyTrait for MyConfig {
///     fn integer_format(&self) -> IntFormat {
///        IntFormat::I32
///     }
/// }
///
/// // If you want to call your method, use disambiguation syntax
///
/// let config = MyConfig;
///
/// let int_format = <MyConfig as MyTrait>::integer_format(&config);
/// assert_eq!(int_format, IntFormat::I32);
/// ```
pub trait Config {
    /// See [IntFormat].
    fn integer_format(&self) -> IntFormat {
        IntFormat::default()
    }

    /// See [FloatFormat].
    fn float_format(&self) -> FloatFormat {
        FloatFormat::default()
    }
}

/// The `Integer Format` configuration option of FEF.
///
/// This configuration option determines how integers are read and written.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntFormat {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
}

/// Default option for `IntFormat` is `I64`.
///
/// # Examples
/// ```rust
/// # use fef::v0::config::IntFormat;
/// assert_eq!(IntFormat::default(), IntFormat::I64);
/// ```
impl Default for IntFormat {
    fn default() -> Self {
        IntFormat::I64
    }
}

/// The `Float Format` configuration option of FEF.
///
/// This configuration option determines how floats are read and written.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFormat {
    F32,
    F64,
}

/// Default option for `FloatFormat` is `F64`.
///
/// # Examples
/// ```rust
/// # use fef::v0::config::FloatFormat;
/// assert_eq!(FloatFormat::default(), FloatFormat::F64);
/// ```
impl Default for FloatFormat {
    fn default() -> Self {
        FloatFormat::F64
    }
}

pub struct OverridableConfig {
    integer_format: Option<IntFormat>,
    float_format: Option<FloatFormat>,
}

impl Config for OverridableConfig {
    fn integer_format(&self) -> IntFormat {
        self.integer_format.unwrap_or_default()
    }

    fn float_format(&self) -> FloatFormat {
        self.float_format.unwrap_or_default()
    }
}

impl OverridableConfig {}

impl Default for OverridableConfig {
    fn default() -> Self {
        OverridableConfig {
            integer_format: None,
            float_format: None,
        }
    }
}
