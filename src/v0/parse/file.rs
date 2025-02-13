use std::io::Read;

use crate::v0::{
    config::{Config, DEFAULT_CONFIG},
    file::{error::FileReadError, File},
    traits::ReadFrom,
};

pub fn read_file<R: ?Sized + Read, C: ?Sized + Config>(
    reader: &mut R,
    configuration: &C,
) -> Result<File, FileReadError> {
    File::read_from(reader, configuration)
}

pub fn read_file_with_default_config<R: ?Sized + Read>(
    reader: &mut R,
) -> Result<File, FileReadError> {
    read_file(reader, &DEFAULT_CONFIG)
}
