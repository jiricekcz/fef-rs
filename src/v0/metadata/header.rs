use std::io::Read;

use crate::{
    common::traits::private::Sealed,
    v0::{raw::VariableLengthEnum, traits::ReadFrom},
};

use super::error::MetadataHeaderReadError;

pub struct MetadataHeader {
    /// Number of records in the metadata
    record_count: u64,
    /// Total number of bytes all the records take. If zero, record_count == 0.
    byte_size: u64,
}
impl Sealed for MetadataHeader {}

impl<R: ?Sized + Read> ReadFrom<R> for MetadataHeader {
    type ReadError = MetadataHeaderReadError;
    fn read_from<C: ?Sized + crate::v0::config::Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let record_count: usize = VariableLengthEnum::read_from(reader, configuration)
            .map_err(|err| MetadataHeaderReadError::RecordCountError(err))?
            .try_into()
            .map_err(|err| MetadataHeaderReadError::RecordCountError(err))?;
        if record_count == 0 {
            return Ok(MetadataHeader {
                byte_size: 0,
                record_count: 0,
            });
        };

        let byte_size: usize = VariableLengthEnum::read_from(reader, configuration)
            .map_err(|err| MetadataHeaderReadError::ByteLengthError(err))?
            .try_into()
            .map_err(|err| MetadataHeaderReadError::ByteLengthError(err))?;

        Ok(MetadataHeader {
            record_count: record_count as u64,
            byte_size: byte_size as u64,
        })
    }
}

impl MetadataHeader {
    pub fn record_count(&self) -> u64 {
        self.record_count
    }

    pub fn byte_size(&self) -> u64 {
        self.byte_size
    }
}
