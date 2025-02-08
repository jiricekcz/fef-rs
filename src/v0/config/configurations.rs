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
