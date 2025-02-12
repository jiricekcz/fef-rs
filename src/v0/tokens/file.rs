use std::io::{Read, Write};

use crate::{
    common::traits::private::Sealed,
    v0::{
        config::Config,
        raw::VariableLengthEnum,
        traits::{ReadFrom, WriteTo},
    },
};

use super::{config, error::FileContentTypeTokenError};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
#[non_exhaustive]
pub enum FileContentTypeToken {
    RawFormula = 0x01,
    SingleFormula = 0x02,
}

impl Into<usize> for FileContentTypeToken {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for FileContentTypeToken {
    type Error = FileContentTypeTokenError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(FileContentTypeToken::RawFormula),
            0x02 => Ok(FileContentTypeToken::SingleFormula),
            _ => Err(FileContentTypeTokenError::IdentifierNotRecognized {
                identifier: value.into(),
            }),
        }
    }
}

impl TryFrom<VariableLengthEnum> for FileContentTypeToken {
    type Error = FileContentTypeTokenError;
    fn try_from(value: VariableLengthEnum) -> Result<Self, Self::Error> {
        let inner: usize = value
            .clone()
            .try_into()
            .map_err(|_| FileContentTypeTokenError::IdentifierTooLarge { identifier: value })?;
        Self::try_from(inner)
    }
}

impl From<FileContentTypeToken> for VariableLengthEnum {
    fn from(value: FileContentTypeToken) -> Self {
        VariableLengthEnum::from(value as usize)
    }
}
impl Sealed for FileContentTypeToken {}
impl<R: ?Sized + Read> ReadFrom<R> for FileContentTypeToken {
    type ReadError = FileContentTypeTokenError;
    fn read_from<C: ?Sized + crate::v0::config::Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<Self, Self::ReadError> {
        let identifier = VariableLengthEnum::read_from(reader, configuration)
            .map_err(FileContentTypeTokenError::ReadError)?;
        Self::try_from(identifier)
    }
}

impl<W: ?Sized + Write> WriteTo<W> for FileContentTypeToken {
    type WriteError = FileContentTypeTokenError;
    fn write_to<C: ?Sized + Config>(
        &self,
        writer: &mut W,
        configuration: &C,
    ) -> Result<(), Self::WriteError> {
        VariableLengthEnum::from(self.to_owned())
            .write_to(writer, configuration)
            .map_err(FileContentTypeTokenError::WriteError)?;
        Ok(())
    }
}
