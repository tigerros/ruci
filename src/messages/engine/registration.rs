use std::fmt::{Display, Formatter, Write};
use crate::auxiliary::{MessageTryFromRawMessageError};
use crate::messages::RawEngineMessage;
use crate::messages::pointers::engine::{EngineMessageParameterPointer, EngineMessagePointer};

/// <https://backscattering.de/chess/uci/#engine-registration>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Registration {
    Checking,
    Ok,
    Error,
}

impl TryFrom<RawEngineMessage> for Registration {
    type Error = MessageTryFromRawMessageError<EngineMessageParameterPointer>;
    
    fn try_from(raw_message: RawEngineMessage) -> Result<Self, MessageTryFromRawMessageError<EngineMessageParameterPointer>> {
        if raw_message.message_pointer != EngineMessagePointer::Registration {
            return Err(MessageTryFromRawMessageError::InvalidMessage);
        }
        
        let Some(value) = raw_message.value else {
            return Err(MessageTryFromRawMessageError::MissingValue);
        };

        match value.as_bytes() {
            b"checking" => Ok(Self::Checking),
            b"ok" => Ok(Self::Ok),
            b"error" => Ok(Self::Error),
            _ => Err(MessageTryFromRawMessageError::ValueParseError),
        }
    }
}

impl Display for Registration {
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