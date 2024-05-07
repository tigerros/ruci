use std::fmt::{Display, Formatter, Write};
use crate::messages::EngineMessage;
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};
use crate::messages::engine::{EngineMessageParameterPointer, EngineMessagePointer};

/// <https://backscattering.de/chess/uci/#engine-registration>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegistrationMessageKind {
    Checking,
    Ok,
    Error,
}

impl TryFrom<RawUciMessage<EngineMessage>> for RegistrationMessageKind {
    type Error = MessageTryFromRawUciMessageError<EngineMessageParameterPointer>;
    
    fn try_from(raw_uci_message: RawUciMessage<EngineMessage>) -> Result<Self, MessageTryFromRawUciMessageError<EngineMessageParameterPointer>> {
        if raw_uci_message.message_pointer != EngineMessagePointer::Registration {
            return Err(MessageTryFromRawUciMessageError::InvalidMessage);
        }
        
        let Some(value) = raw_uci_message.value else {
            return Err(MessageTryFromRawUciMessageError::MissingValue);
        };

        match value.as_bytes() {
            b"checking" => Ok(Self::Checking),
            b"ok" => Ok(Self::Ok),
            b"error" => Ok(Self::Error),
            _ => Err(MessageTryFromRawUciMessageError::ValueParseError),
        }
    }
}

impl Display for RegistrationMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("registration ")?;
        
        match self {
            Self::Checking => f.write_str("checking")?,
            Self::Ok => f.write_str("ok")?,
            Self::Error => f.write_str("error")?,
        }

        f.write_char('\n')
    }
}