use core::fmt::{Display, Formatter};
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::MessageParseErrorKind;
use super::{pointers, traits};

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

impl_message!(Registration);
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

    Err(MessageParseError {
        expected: "checking, ok or error",
        kind: MessageParseErrorKind::ValueParseError
    })
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
    use super::Registration;
    use crate::{MessageParseError, MessageParseErrorKind};
    use crate::dev_macros::{assert_from_str_message, assert_message_to_from_str, assert_message_to_str};

    #[test]
    fn to_from_str_checking() {
        assert_message_to_from_str!(engine Registration::Checking, "registration checking");
    }

    #[test]
    fn to_from_str_ok() {
        assert_message_to_from_str!(engine Registration::Ok, "registration ok");
    }

    #[test]
    fn to_from_str_error() {
        let m = Registration::Error;

        assert_from_str_message!(
            engine
            "registration tes error\n",
            Ok(m)
        );
        assert_message_to_str!(engine m, "registration error");
    }

    #[test]
    fn parse_error() {
        assert_from_str_message!(engine "registration invalid", Err::<Registration, MessageParseError>(MessageParseError {
            expected: "checking, ok or error",
            kind: MessageParseErrorKind::ValueParseError
        }));
    }
}