use std::io::Read;

use crate::v0::{
    config::{Config, DEFAULT_CONFIG},
    file::{error::FileReadError, File},
    traits::ReadFrom,
};

/// Reads a [file](https://github.com/jiricekcz/fef-specification/blob/main/README.md) from a reader to memory.
///
/// This is a convenience method that reads a file from a reader entirely into memory. It is supposed
/// to suit most use cases, but if you need more control over the reading process, you will probably have
/// to compose the reading process yourself from other parsing functions.
///
/// Note, that this function expects the version to have already been read from the reader.
///
/// This method outputs a [`File`] enum, which contains the parsed file.
pub fn read_file<R: ?Sized + Read, C: ?Sized + Config>(
    reader: &mut R,
    configuration: &C,
) -> Result<File, FileReadError> {
    File::read_from(reader, configuration)
}

/// Reads a [file](https://github.com/jiricekcz/fef-specification/blob/main/README.md) from a reader to memory using the default configuration.
///
/// This is useful when dealing with arbitrary files, that expect default configuration.
///
/// Note, that this function expects the version to have already been read from the reader.
/// For more information, see the [`read_file`] function.
pub fn read_file_with_default_config<R: ?Sized + Read>(
    reader: &mut R,
) -> Result<File, FileReadError> {
    read_file(reader, &DEFAULT_CONFIG)
}
