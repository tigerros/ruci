use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::raw_message::RawMessage;

/// <https://backscattering.de/chess/uci/#engine-registration>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Registration {
    Checking,
    Ok,
    Error,
}

impl TryFrom<RawMessage> for Registration {
    type Error = MessageParseError;
    
    fn try_from(raw_message: RawMessage) -> Result<Self, MessageParseError> {
        if raw_message.message_pointer != super::pointers::MessagePointer::Registration.into() {
            return Err(MessageParseError::InvalidMessage);
        }
        
        let Some(value) = raw_message.value else {
            return Err(MessageParseError::MissingValue);
        };

        match value.as_bytes() {
            b"checking" => Ok(Self::Checking),
            b"ok" => Ok(Self::Ok),
            b"error" => Ok(Self::Error),
            _ => Err(MessageParseError::ValueParseError),
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