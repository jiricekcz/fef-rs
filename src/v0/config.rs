//! Configuration of the FEF parser.

use crate::common::traits::private::Sealed;

/// Configuration requirements for a FEF parser.
pub trait Config: Sealed {
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
