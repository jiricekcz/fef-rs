use std::{convert::Infallible, io::Write};

use crate::v0::{
    config::Config,
    metadata::{
        error::{FromIteratorMetadataWriteError, MetadataWriteError},
        MetadataHeader, MetadataRecord,
    },
    traits::WriteTo,
};

impl<E: std::error::Error> From<Infallible> for FromIteratorMetadataWriteError<E> {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
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
