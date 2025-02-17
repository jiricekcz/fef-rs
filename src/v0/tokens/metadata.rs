use crate::v0::raw::VariableLengthEnum;

use super::error::MetadataTokenError;

/// Metadata key identifiers.
///
/// Representation of this enum is not specified. This would enable the addition of a custom memory representation, when rust supports it.
#[non_exhaustive]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum MetadataToken {
    Name,
    VariableName,
    ReservedOfficial(u32),
    ReservedThirdParty(u32),
    ReservedCustom(u32),
}

impl Into<usize> for MetadataToken {
    fn into(self) -> usize {
        match self {
            MetadataToken::Name => 0x01,
            MetadataToken::VariableName => 0x02,
            MetadataToken::ReservedOfficial(id) => id as usize,
            MetadataToken::ReservedThirdParty(id) => id as usize,
            MetadataToken::ReservedCustom(id) => id as usize,
        }
    }
}

impl TryFrom<usize> for MetadataToken {
    type Error = MetadataTokenError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(MetadataToken::Name),
            0x02 => Ok(MetadataToken::VariableName),
            id @ 0x0..0x40000 => Ok(MetadataToken::ReservedOfficial(id as u32)),
            id @ 0x40000..0x100000 => Ok(MetadataToken::ReservedThirdParty(id as u32)),
            id @ 0x100000..0x200000 => Ok(MetadataToken::ReservedCustom(id as u32)),
            _ => Err(MetadataTokenError::IdentifierNotRecognized {
                identifier: value.into(),
            }),
        }
    }
}

impl TryFrom<VariableLengthEnum> for MetadataToken {
    type Error = MetadataTokenError;
    fn try_from(value: VariableLengthEnum) -> Result<Self, Self::Error> {
        let inner: usize = value
            .clone()
            .try_into()
            .map_err(|_| MetadataTokenError::IdentifierTooLarge { identifier: value })?;
        Self::try_from(inner)
    }
}
