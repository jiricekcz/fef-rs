pub mod error;
mod header;
mod record;
mod records;
pub(crate) mod traits;

pub use header::MetadataHeader;
pub use record::MetadataRecord;
pub use records::*;
