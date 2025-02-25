use crate::{
    common::traits::private::Sealed,
    v0::{raw::VariableLengthEnum, tokens::ConfigToken},
};

/// Configuration requirements for a FEF parser.
pub trait Config {}

pub(crate) trait EnumConfiguration:
    Sealed + Copy + Default + Eq + TryFrom<VariableLengthEnum>
{
    fn token() -> ConfigToken;

    fn value(&self) -> usize;
}
