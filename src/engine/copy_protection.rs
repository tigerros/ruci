use core::fmt::{Display, Formatter};
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::MessageParseErrorKind;
use super::{pointers, traits};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Engine's copy protection. GUIs should respect this.
///
/// Sent after [`UciOk`](super::UciOk).
///
/// <https://backscattering.de/chess/uci/#engine-copyprotection>
pub enum CopyProtection {
    Ok,
    Error,
}

impl_message!(CopyProtection);
message_from_impl!(engine CopyProtection);
from_str_parts!(impl CopyProtection for parts -> Result {
    for part in parts {
        // TODO: Should it be lowercased (case insensitive)?
        match part.trim() {
            "ok" => return Ok(Self::Ok),
            "error" => return Ok(Self::Error),
            _ => ()
        }
    }

    Err(MessageParseError {
        expected: "ok or error",
        kind: MessageParseErrorKind::ValueParseError
    })
});

impl Display for CopyProtection {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("copyprotection ")?;
        
        match self {
            Self::Ok => f.write_str("ok")?,
            Self::Error => f.write_str("error")?,
        }
        
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::CopyProtection;
    use alloc::string::ToString;
    use crate::{MessageParseError, MessageParseErrorKind};
    use crate::dev_macros::{assert_from_str_message, assert_message_to_from_str};

    #[test]
    fn to_from_str_ok() {
        let m = CopyProtection::Ok;
        let str = "copyprotection ok";
        assert_message_to_from_str!(engine m, str);
    }

    #[test]
    fn to_from_str_error() {
        let m = CopyProtection::Error;
        let str = "copyprotection error";
        assert_message_to_from_str!(engine m, str);
    }

    #[test]
    fn parse_error() {
        assert_from_str_message!(engine "copyprotection why   \t are you here? 🤨\n\n", Err::<CopyProtection, MessageParseError>(MessageParseError {
            expected: "ok or error",
            kind: MessageParseErrorKind::ValueParseError,
        }));
    }
}