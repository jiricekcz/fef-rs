use std::io::Read;

use crate::v0::{
    config::Config,
    metadata::{
        error::{MetadataHeaderReadError, MetadataReadError, MetadataRecordReadError},
        MetadataHeader, MetadataRecord,
    },
    traits::ReadFrom,
};

/// Reads [metadata](https://github.com/jiricekcz/fef-specification/blob/main/metadata/Metadata.md) from a byte stream and returns it as an iterator.
///
/// For most use cases, you will want to use the [`parse_metadata_as_vec`] function instead.
///
/// # Example
/// ```rust
/// # use fef::v0::parse::parse_metadata;
/// # use fef::v0::config::DEFAULT_CONFIG;
/// # use fef::v0::metadata::MetadataRecord;
/// # use fef::v0::raw::VariableLengthEnum;
/// # use fef::v0::metadata::NameMetadataRecordObj;
/// # use fef::v0::metadata::VariableNameMetadataRecordObj;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let bytes: Vec<u8> = vec![
///     0x02, // 2 records
///     0x13, // together 19 bytes
///     0x01, // Name record
///         0x08, // Total name record length
///         0x07, // String length
///             b'F', b'o', b'r', b'm', b'u', b'l', b'a', // "Formula"
///     0x02, // Variable name record
///         0x03, // Total variable name record length
///         0x01, // Variable with ID 1
///         0x01, // String length
///             b'x', // "x"
///     0x00, 0x00, 0x00, 0x00 // Padding
/// ];
///
/// let mut reader = &mut bytes.as_slice();
/// let mut metadata = parse_metadata(&mut reader, &DEFAULT_CONFIG)?;
///
///
/// assert_eq!(metadata.next().ok_or("first record exists")??, MetadataRecord::Name(
///     NameMetadataRecordObj::new(
///        "Formula".to_string()
///    )
/// ));
///
/// assert_eq!(metadata.next().ok_or("second record exists")??, MetadataRecord::VariableName(
///     VariableNameMetadataRecordObj::new(
///         "x".to_string(),
///         VariableLengthEnum::from(1)
///     )
/// ));
///
/// assert!(metadata.next().is_none()); // No more records
///
/// drop(metadata); // dropping the iterator to access reader again
///
/// assert!(reader.is_empty()); // Padding was read and disregarded
/// # Ok(())
/// # }
pub fn parse_metadata<'a, 'b, R: ?Sized + Read, C: ?Sized + Config>(
    reader: &'a mut R,
    configuration: &'b C,
) -> Result<
    impl Iterator<Item = Result<MetadataRecord, MetadataRecordReadError>> + use<'a, 'b, R, C>,
    MetadataHeaderReadError,
> {
    MetadataIterator::new(reader, configuration)
}

struct MetadataIterator<'a, 'b, R: ?Sized + Read, C: ?Sized + Config> {
    limited_reader: std::io::Take<&'a mut R>,
    configuration: &'b C,
    records_remaining: usize,
}

impl<'a, 'b, R: ?Sized + Read, C: ?Sized + Config> Iterator for MetadataIterator<'a, 'b, R, C> {
    type Item = Result<MetadataRecord, MetadataRecordReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.records_remaining == 0 {
            return None;
        }
        self.records_remaining -= 1;
        Some(MetadataRecord::read_from(
            &mut self.limited_reader,
            self.configuration,
        ))
    }
}

impl<'a, 'b, R: ?Sized + Read, C: ?Sized + Config> MetadataIterator<'a, 'b, R, C> {
    pub(crate) fn new(
        reader: &'a mut R,
        configuration: &'b C,
    ) -> Result<MetadataIterator<'a, 'b, R, C>, MetadataHeaderReadError> {
        let header = MetadataHeader::read_from(reader, configuration)?;
        Ok(MetadataIterator {
            limited_reader: reader.take(header.byte_size() as u64),
            configuration,
            records_remaining: header.record_count(),
        })
    }
}

impl<'a, 'b, R: ?Sized + Read, C: ?Sized + Config> Drop for MetadataIterator<'a, 'b, R, C> {
    fn drop(&mut self) {
        let mut buf: Vec<u8> = Vec::new();
        let _ = self.limited_reader.read_to_end(&mut buf);
    }
}

/// Reads [metadata](https://github.com/jiricekcz/fef-specification/blob/main/metadata/Metadata.md) from a byte stream and returns it as a vector.
///
/// The generic [`parse_metadata`] function parses metadata as a lazy iterator. That can be useful, however most of the time you will want to read
/// all the metadata at once. This function does exactly that.
///
/// # Example
/// ```rust
/// # use fef::v0::parse::parse_metadata_as_vec;
/// # use fef::v0::config::DEFAULT_CONFIG;
/// # use fef::v0::metadata::MetadataRecord;
/// # use fef::v0::raw::VariableLengthEnum;
/// # use fef::v0::metadata::NameMetadataRecordObj;
/// # use fef::v0::metadata::VariableNameMetadataRecordObj;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let bytes: Vec<u8> = vec![
///     0x02, // 2 records
///     0x13, // together 5 bytes
///     0x01, // Name record
///         0x08, // Total name record length
///         0x07, // String length
///             b'F', b'o', b'r', b'm', b'u', b'l', b'a', // "Formula"
///     0x02, // Variable name record
///         0x03, // Total variable name record length
///         0x01, // Variable with ID 1
///         0x01, // String length
///             b'x', // "x"
///     0x00, 0x00, 0x00, 0x00 // Padding
/// ];
///
/// let mut reader = &mut bytes.as_slice();
/// let metadata = parse_metadata_as_vec(&mut reader, &DEFAULT_CONFIG)?;
///
/// assert_eq!(metadata.len(), 2);
///
/// // Metadata are in order
/// assert_eq!(metadata[0], MetadataRecord::Name(
///     NameMetadataRecordObj::new(
///        "Formula".to_string()
///    )
/// ));
///
/// assert_eq!(metadata[1], MetadataRecord::VariableName(
///     VariableNameMetadataRecordObj::new(
///         "x".to_string(),
///         VariableLengthEnum::from(1)
///     )
/// ));
///
/// assert!(reader.is_empty()); // Padding was read and disregarded
/// # Ok(())
/// # }
pub fn parse_metadata_as_vec<R: ?Sized + Read, C: ?Sized + Config>(
    reader: &mut R,
    configuration: &C,
) -> Result<Vec<MetadataRecord>, MetadataReadError> {
    let mut records = Vec::new();
    for record in parse_metadata(reader, configuration)? {
        records.push(record?);
    }
    Ok(records)
}
