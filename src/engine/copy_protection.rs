use core::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, message_from_impl};

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

message_from_impl!(engine CopyProtection);
from_str_parts!(impl CopyProtection for parts -> Result<Self, MessageParseError> {
    for part in parts {
        match part.trim().to_lowercase().as_str() {
            "ok" => return Ok(Self::Ok),
            "error" => return Ok(Self::Error),
            _ => ()
        }
    }

    Err(MessageParseError::ValueParseError { expected: "ok or error" })
});

impl Display for CopyProtection {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
    use core::str::FromStr;
    use crate::Message;
    use super::CopyProtection;
    use alloc::string::ToString;
    use crate::MessageParseError;
    use pretty_assertions::{assert_eq, assert_matches};

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
        assert_matches!(Message::from_str("copyprotection why   \t are you here? 🤨\n\n"), Err(MessageParseError::ValueParseError { .. }));
    }
}