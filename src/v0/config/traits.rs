use crate::{
    common::traits::private::Sealed,
    v0::{raw::VariableLengthEnum, tokens::ConfigToken},
};

/// Configuration requirements for a FEF parser.
pub trait Config {}
