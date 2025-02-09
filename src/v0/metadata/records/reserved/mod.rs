mod custom;
mod official;
mod third_party;

pub use custom::CustomReservedMetadataRecord;
pub use official::OfficialReservedMetadataRecord;
pub use third_party::ThirdPartyReservedMetadataRecord;

pub enum ReservedMetadataRecord {
    Official(OfficialReservedMetadataRecord),
    Custom(CustomReservedMetadataRecord),
    ThirdParty(ThirdPartyReservedMetadataRecord),
}
