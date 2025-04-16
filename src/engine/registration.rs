use core::fmt::{Display, Formatter};
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};

/// Engine's registration status.
///
/// Sent after [`Uci`](crate::gui::Uci) or [`Register`](crate::gui::Register).
///
/// <https://backscattering.de/chess/uci/#engine-registration>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Registration {
    Checking,
    Ok,
    Error,
}

impl_message!(copy Registration);
message_from_impl!(engine Registration);
from_str_parts!(impl Registration for parts -> Result {
    for part in parts {
        match part.trim() {
            "checking" => return Ok(Self::Checking),
            "ok" => return Ok(Self::Ok),
            "error" => return Ok(Self::Error),
            _ => ()
        }
    }

    Err(MessageParseError::ValueParseError { expected: "checking, ok or error" })
});

impl Display for Registration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("registration ")?;
        
        match self {
            Self::Checking => f.write_str("checking")?,
            Self::Ok => f.write_str("ok")?,
            Self::Error => f.write_str("error")?,
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;
    use core::str::FromStr;
    use crate::{engine, Message};
    use super::Registration;
    use crate::MessageParseError;
    use pretty_assertions::{assert_eq, assert_matches};

    #[test]
    fn to_from_str_checking() {
        let repr: engine::Message = Registration::Checking.into();
        let str = "registration checking";
        assert_eq!(repr.to_string(), str);
        assert_eq!(engine::Message::from_str(str).unwrap(), repr);
    }

    #[test]
    fn to_from_str_ok() {
        let repr: Message = Registration::Ok.into();
        let str = "registration ok";
        assert_eq!(repr.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), repr);
    }

    #[test]
    fn to_from_str_error() {
        let repr: Message = Registration::Error.into();
        assert_eq!(repr.to_string(), "registration error");
        assert_eq!(Message::from_str("registration tes error\n").unwrap(), repr);
    }

    #[test]
    fn parse_error() {
        assert_matches!(Message::from_str("registration invalid"), Err(MessageParseError::ValueParseError { .. }));
    }
}