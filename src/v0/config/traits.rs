use crate::common::traits::private::Sealed;

use super::configurations::*;

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
