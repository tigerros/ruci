use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// <https://backscattering.de/chess/uci/#engine-copyprotection>
pub enum CopyProtection {
    Ok,
    Error,
}

message_from_impl!(engine CopyProtection);

impl TryFrom<RawMessage> for CopyProtection {
    type Error = MessageParseError;

    fn try_from(raw_message: RawMessage) -> Result<Self, MessageParseError> {
        if raw_message.message_pointer != super::pointers::MessagePointer::CopyProtection.into() {
            return Err(MessageParseError::InvalidMessage);
        }

        let Some(value) = raw_message.value else {
            return Err(MessageParseError::MissingValue);
        };

        match value.as_bytes() {
            b"ok" => Ok(Self::Ok),
            b"error" => Ok(Self::Error),
            _ => Err(MessageParseError::ValueParseError),
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::str::FromStr;
    use crate::Message;
    use super::*;

    #[test]
    fn to_from_str_ok() {
        let m: Message = CopyProtection::Ok.into();
        let str = "copyprotection ok\n";
        assert_eq!(m.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), m);
    }

    #[test]
    fn to_from_str_error() {
        let m: Message = CopyProtection::Error.into();
        let str = "copyprotection error\n";
        assert_eq!(m.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), m);
    }

    #[test]
    fn parse_error() {
        pretty_assertions::assert_eq!(Message::from_str("copyprotection why   \t are you here? 🤨\n\n"), Err(MessageParseError::ValueParseError));
    }
}