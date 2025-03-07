use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;

/// <https://backscattering.de/chess/uci/#engine-registration>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Registration {
    Checking,
    Ok,
    Error,
}

message_from_impl!(engine Registration);

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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::str::FromStr;
    use crate::Message;
    use super::*;

    #[test]
    fn to_from_str_checking() {
        let repr: Message = Registration::Checking.into();
        let str = "registration checking\n";
        assert_eq!(repr.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), repr);
    }

    #[test]
    fn to_from_str_ok() {
        let repr: Message = Registration::Ok.into();
        let str = "registration ok\n";
        assert_eq!(repr.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), repr);
    }

    #[test]
    fn to_from_str_error() {
        let repr: Message = Registration::Error.into();
        let str = "registration error\n";
        assert_eq!(repr.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), repr);
    }

    #[test]
    fn parse_error() {
        assert_eq!(Message::from_str("registration invalid\n"), Err(MessageParseError::ValueParseError));
    }
}