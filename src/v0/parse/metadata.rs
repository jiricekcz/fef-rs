use std::io::Read;

use crate::v0::{
    config::Config,
    metadata::{
        error::{MetadataHeaderReadError, MetadataRecordReadError},
        MetadataHeader, MetadataRecord,
    },
    traits::ReadFrom,
};

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
