//! Public and sealed traits
use std::io::Bytes;

pub(crate) mod private {
    pub trait Sealed {}
}

/// Trait for reading a value from bytes with option to fail.
pub trait ReadFrom<T>: private::Sealed + Sized
where
    T: std::io::Read,
{
    /// The error type that can be returned when reading fails.
    type ReadError: std::error::Error;

    /// Reads a value from the given reader.
    fn read_from_bytes(reader: &mut Bytes<T>) -> Result<Self, Self::ReadError>;
}
