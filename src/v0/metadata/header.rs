use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        raw::VariableLengthEnum,
        traits::{ReadFrom, WriteTo},
    },
};

use super::error::{MetadataHeaderReadError, MetadataHeaderWriteError};

/// Header for the metadata section of a FEF file.
pub struct MetadataHeader {
    /// Number of records in the metadata
    record_count: usize,
    /// Total number of bytes all the records take. If zero, record_count == 0.
    byte_size: usize,
}
impl Sealed for MetadataHeader {}

impl<R: ?Sized + Read> ReadFrom<R> for MetadataHeader {
    type ReadError = MetadataHeaderReadError;

    /// Reads a metadata header from a reader.
    ///
    /// # Example
    ///
    /// Reading a non-empty metadata header:
    /// ```rust
    /// # use fef::v0::metadata::MetadataHeader;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::ReadFrom;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let data: Vec<u8> = vec![
    ///     0x02, // 2 records
    ///     0x10, // takes 16 bytes
    ///     0x01, 0x02, 0x01, b'F', // record 1 - formula name "F"
    ///     0x02, 0x03, 0x01, 0x01, b'x', // record 2 - variable name "x"
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 // padding
    /// ];
    /// let mut reader = &mut data.as_slice();
    /// let header = MetadataHeader::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// assert_eq!(header.record_count(), 2);
    /// assert_eq!(header.byte_size(), 16);
    ///
    /// # let record1 = fef::v0::metadata::MetadataRecord::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// # let record2 = fef::v0::metadata::MetadataRecord::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// # assert_eq!(record1, fef::v0::metadata::MetadataRecord::Name(fef::v0::metadata::NameMetadataRecordObj::new("F".to_string())));
    /// # assert_eq!(record2, fef::v0::metadata::MetadataRecord::VariableName(fef::v0::metadata::VariableNameMetadataRecordObj::new("x".to_string(), fef::v0::raw::VariableLengthEnum::from(1))));
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Reading an empty metadata header:
    /// ```rust
    /// # use fef::v0::metadata::MetadataHeader;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::ReadFrom;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let data: Vec<u8> = vec![
    ///    0x00, // 0 records
    /// ];
    /// let mut reader = &mut data.as_slice();
    /// let header = MetadataHeader::read_from(&mut reader, &DEFAULT_CONFIG)?;
    /// assert_eq!(header.record_count(), 0);
    /// assert_eq!(header.byte_size(), 0);
    /// # Ok(())
    /// # }
    /// ```
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

    /// Writes the metadata header to a writer.
    ///
    /// # Example
    ///
    /// Writing a non-empty metadata header:
    /// ```rust
    /// # use fef::v0::metadata::MetadataHeader;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::WriteTo;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let header = MetadataHeader::new(2, 16);
    /// let mut writer: Vec<u8> = Vec::new();
    /// header.write_to(&mut writer, &DEFAULT_CONFIG)?;
    /// assert_eq!(writer, vec![0x02, 0x10]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Writing an empty metadata header:
    /// ```rust
    /// # use fef::v0::metadata::MetadataHeader;
    /// # use fef::v0::config::DEFAULT_CONFIG;
    /// # use fef::v0::traits::WriteTo;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let header = MetadataHeader::new(0, 0);
    /// let mut writer: Vec<u8> = Vec::new();
    /// header.write_to(&mut writer, &DEFAULT_CONFIG)?;
    /// assert_eq!(writer, vec![0x00]);
    /// # Ok(())
    /// # }
    /// ```
    fn write_to<C: ?Sized + crate::v0::config::Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        VariableLengthEnum::from(self.record_count as usize)
            .write_to(writer, configuration)
            .map_err(|err| MetadataHeaderWriteError::RecordCountError(err))?;
        if self.record_count != 0 {
            VariableLengthEnum::from(self.byte_size as usize)
                .write_to(writer, configuration)
                .map_err(|err| MetadataHeaderWriteError::ByteLengthError(err))?;
        }
        Ok(())
    }
}

impl MetadataHeader {
    /// Returns the number of records in the metadata.
    pub fn record_count(&self) -> usize {
        self.record_count
    }

    /// Returns the number of bytes the metadata section takes including padding bytes at the end.
    pub fn byte_size(&self) -> usize {
        self.byte_size
    }

    /// Creates a new metadata header from the given record count and byte size.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fef::v0::metadata::MetadataHeader;
    /// let header = MetadataHeader::new(10, 100);
    ///
    /// assert_eq!(header.record_count(), 10);
    /// assert_eq!(header.byte_size(), 100);
    /// ```
    pub fn new(record_count: usize, byte_size: usize) -> Self {
        MetadataHeader {
            record_count,
            byte_size,
        }
    }
}
