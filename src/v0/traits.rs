//! Public and sealed traits

use crate::{common::traits::private::Sealed, v0::config::Config};

/// Trait for reading a value from bytes with option to fail.
pub trait ReadFrom<T>: Sealed + Sized
where
    T: std::io::Read + ?Sized,
{
    /// The error type that can be returned when reading fails.
    type ReadError: std::error::Error;

    /// Reads a value from the given reader.
    fn read_from<C: ?Sized + Config>(
        reader: &mut T,
        configuration: &C,
    ) -> Result<Self, Self::ReadError>;
}

pub trait WriteTo<W>: Sealed
where
    W: std::io::Write + ?Sized,
{
    type WriteError: std::error::Error;

    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError>;
}
