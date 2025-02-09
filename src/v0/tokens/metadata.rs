#[non_exhaustive]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum MetadataToken {
    Name,
    VariableName,
    ReservedOfficial(u32),
    ReservedThirdParty(u32),
    ReservedCustom(u32),
}
