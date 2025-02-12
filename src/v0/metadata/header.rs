use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        raw::VariableLengthEnum,
        traits::{ReadFrom, WriteTo},
    },
};

use super::error::{MetadataHeaderReadError, MetadataHeaderWriteError};

pub struct MetadataHeader {
    /// Number of records in the metadata
    record_count: usize,
    /// Total number of bytes all the records take. If zero, record_count == 0.
    byte_size: usize,
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
            record_count: record_count,
            byte_size: byte_size,
        })
    }
}

impl<W: ?Sized + Write> WriteTo<W> for MetadataHeader {
    type WriteError = MetadataHeaderWriteError;
    fn write_to<C: ?Sized + crate::v0::config::Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        VariableLengthEnum::from(self.record_count as usize)
            .write_to(writer, configuration)
            .map_err(|err| MetadataHeaderWriteError::RecordCountError(err))?;
        VariableLengthEnum::from(self.byte_size as usize)
            .write_to(writer, configuration)
            .map_err(|err| MetadataHeaderWriteError::ByteLengthError(err))
    }
}

impl MetadataHeader {
    pub fn record_count(&self) -> usize {
        self.record_count
    }

    pub fn byte_size(&self) -> usize {
        self.byte_size
    }

    pub(crate) fn new(record_count: usize, byte_size: usize) -> Self {
        MetadataHeader {
            record_count,
            byte_size,
        }
    }
}
