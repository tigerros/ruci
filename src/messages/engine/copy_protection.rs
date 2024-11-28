use std::fmt::{Display, Formatter, Write};
use crate::{MessageTryFromRawMessageError};
use crate::messages::{RawEngineMessage, EngineMessageParameterPointer, EngineMessagePointer};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-copyprotection>
pub enum CopyProtection {
    Ok,
    Error,
}

impl TryFrom<RawEngineMessage> for CopyProtection {
    type Error = MessageTryFromRawMessageError<EngineMessageParameterPointer>;

    fn try_from(raw_message: RawEngineMessage) -> Result<Self, MessageTryFromRawMessageError<EngineMessageParameterPointer>> {
        if raw_message.message_pointer != EngineMessagePointer::CopyProtection {
            return Err(MessageTryFromRawMessageError::InvalidMessage);
        }

        let Some(value) = raw_message.value else {
            return Err(MessageTryFromRawMessageError::MissingValue);
        };

        match value.as_bytes() {
            b"ok" => Ok(Self::Ok),
            b"error" => Ok(Self::Error),
            _ => Err(MessageTryFromRawMessageError::ValueParseError),
        }
    }
}

impl Display for CopyProtection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("copyprotection ")?;
        match self {
            Self::Ok => f.write_str("ok")?,
            Self::Error => f.write_str("error")?,
        }
        f.write_char('\n')
    }
}