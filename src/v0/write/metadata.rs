use std::{convert::Infallible, io::Write};

use crate::v0::{
    config::Config,
    metadata::{
        error::{FromIteratorMetadataWriteError, MetadataWriteError},
        MetadataHeader, MetadataRecord,
    },
    traits::WriteTo,
};

/// Writes metadata to a byte stream.
///
/// Reads from an iterator of metadata records and writes them to a writer.
/// Most of the time, you will want to use [`write_metadata_from_vec`] instead,
/// as it is more convenient.
pub fn write_metadata<
    'a,
    W: ?Sized + Write,
    C: ?Sized + Config,
    E: std::error::Error,
    I: Iterator<Item = Result<&'a MetadataRecord, E>>,
>(
    writer: &mut W,
    configuration: &C,
    records: I,
    record_count: usize,
    byte_size: usize,
) -> Result<(), FromIteratorMetadataWriteError<E>> {
    let header = MetadataHeader::new(record_count, byte_size);
    header
        .write_to(writer, configuration)
        .map_err(MetadataWriteError::HeaderError)
        .map_err(FromIteratorMetadataWriteError::MetadataWriteError)?;
    for record in records {
        record
            .map_err(FromIteratorMetadataWriteError::IteratorError)?
            .write_to(writer, configuration)
            .map_err(MetadataWriteError::RecordError)?;
    }
    Ok(())
}

/// Writes metadata from a [`Vec`] to a byte stream.
///
/// This is a convenience function that writes metadata from a [`Vec`] to a byte stream.
/// It calculates the number of records and the byte length of the records for you.
///
/// # Example
///
/// ```rust
/// # use fef::v0::write::write_metadata_from_vec;
/// # use fef::v0::config::DEFAULT_CONFIG;
/// # use fef::v0::metadata::MetadataRecord;
/// # use fef::v0::metadata::VariableNameMetadataRecordObj;
/// # use fef::v0::metadata::NameMetadataRecordObj;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let name_record: MetadataRecord = NameMetadataRecordObj::new("Formula".to_string()).into();
/// let variable_name_record: MetadataRecord = VariableNameMetadataRecordObj::new("x".to_string(), 1.into()).into();
///
/// let records: Vec<MetadataRecord> = vec![name_record, variable_name_record];
///
/// let mut writer = Vec::new();
/// write_metadata_from_vec(&mut writer, &DEFAULT_CONFIG, &records)?;
///
/// let expected_result: Vec<u8> = vec![
///     0x02, // 2 records
///     0x0B, // together 5 bytes
///     0x01, // Name record
///         0x08, // Total name record length
///         0x07, // String length
///             b'F', b'o', b'r', b'm', b'u', b'l', b'a', // "Formula"
///     0x02, // Variable name record
///         0x03, // Total variable name record length
///         0x01, // Variable with ID 1
///         0x01, // String length
///             b'x', // "x"
/// ];
///
/// assert_eq!(writer, expected_result);
/// # Ok(())
/// # }
pub fn write_metadata_from_vec<W: ?Sized + Write, C: ?Sized + Config>(
    writer: &mut W,
    configuration: &C,
    records: &Vec<MetadataRecord>,
) -> Result<(), MetadataWriteError> {
    let record_count = records.len();
    let byte_length = records.iter().map(MetadataRecord::byte_length).sum();
    write_metadata(
        writer,
        configuration,
        records
            .into_iter()
            .map(|record| Result::<&MetadataRecord, Infallible>::Ok(record)),
        record_count,
        byte_length,
    )
    .map_err(|err| match err {
        FromIteratorMetadataWriteError::MetadataWriteError(err) => err,
    })?;
    Ok(())
}
