use super::{ReservedMetadataRecord, UnknownMetadataRecord};

#[non_exhaustive]
pub enum MetadataRecord {
    Reserved(ReservedMetadataRecord),
    Unknown(UnknownMetadataRecord),
}
