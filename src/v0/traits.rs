//! Public and sealed traits

use crate::v0::config::Config;

pub(crate) mod private {
    pub trait Sealed {}
}

/// Trait for reading a value from bytes with option to fail.
pub trait ReadFrom<T>: private::Sealed + Sized
where
    T: std::io::Read + ?Sized,
{
    /// The error type that can be returned when reading fails.
    type ReadError: std::error::Error;

    /// Reads a value from the given reader.
    fn read_from<C: Config>(reader: &mut T, configuration: &C) -> Result<Self, Self::ReadError>;
}
