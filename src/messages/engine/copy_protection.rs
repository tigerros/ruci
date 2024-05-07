use std::fmt::{Display, Formatter, Write};
use crate::messages::EngineMessage;
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};
use crate::messages::engine::{EngineMessageParameterPointer, EngineMessagePointer};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-copyprotection>
pub enum CopyProtectionMessageKind {
    Ok,
    Error,
}

impl TryFrom<RawUciMessage<EngineMessage>> for CopyProtectionMessageKind {
    type Error = MessageTryFromRawUciMessageError<EngineMessageParameterPointer>;

    fn try_from(raw_uci_message: RawUciMessage<EngineMessage>) -> Result<Self, MessageTryFromRawUciMessageError<EngineMessageParameterPointer>> {
        if raw_uci_message.message_pointer != EngineMessagePointer::CopyProtection {
            return Err(MessageTryFromRawUciMessageError::InvalidMessage);
        }

        let Some(value) = raw_uci_message.value else {
            return Err(MessageTryFromRawUciMessageError::MissingValue);
        };

        match value.as_bytes() {
            b"ok" => Ok(Self::Ok),
            b"error" => Ok(Self::Error),
            _ => Err(MessageTryFromRawUciMessageError::ValueParseError),
        }
    }
}

impl Display for CopyProtectionMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("copyprotection ")?;
        match self {
            Self::Ok => f.write_str("ok")?,
            Self::Error => f.write_str("error")?,
        }
        f.write_char('\n')
    }
}