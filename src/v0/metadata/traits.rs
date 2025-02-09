use crate::{
    common::traits::private::Sealed,
    v0::tokens::{error::MetadataTokenError, MetadataToken},
};

pub trait MetadataRecordObj: Sealed {
    fn token(&self) -> Result<MetadataToken, MetadataTokenError>;
    fn byte_length(&self) -> usize;
}
