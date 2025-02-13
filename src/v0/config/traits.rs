use crate::{
    common::traits::private::Sealed,
    v0::{raw::VariableLengthEnum, tokens::ConfigToken},
};

use super::configurations::*;

/// Configuration requirements for a FEF parser.
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

pub(crate) trait EnumConfiguration:
    Sealed + Copy + Default + Eq + TryFrom<VariableLengthEnum>
{
    fn token() -> ConfigToken;

    fn value(&self) -> usize;
}
