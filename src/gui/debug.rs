use core::fmt::{Display, Formatter};
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use super::{pointers, traits};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Tells the engine to be in debug mode.
///
/// <https://backscattering.de/chess/uci/#gui-debug>
pub struct Debug(pub bool);

impl_message!(Debug);
message_from_impl!(gui Debug);
from_str_parts!(impl Debug for parts -> Result {
    for part in parts {
        match part.trim() {
            "on" => return Ok(Self(true)),
            "off" => return Ok(Self(false)),
            _ => ()
        }
    }

    Err(MessageParseError::ValueParseError { expected: "on or off" })
});

impl Display for Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("debug ")?;
        
        if self.0 {
            f.write_str("on")
        } else {
            f.write_str("off")
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;
    use core::str::FromStr;
    use crate::{gui, Message};
    use super::Debug;
    use crate::MessageParseError;
    use pretty_assertions::{assert_eq, assert_matches};

    #[test]
    fn to_from_str_on() {
        let m: Message = Debug(true).into();
        let str = "debug on";
        assert_eq!(m.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), m);
    }

    #[test]
    fn to_from_str_off() {
        let m: gui::Message = Debug(false).into();
        let str = "debug off";
        assert_eq!(m.to_string(), str);
        assert_eq!(gui::Message::from_str(str).unwrap(), m);

        let m: gui::Message = Debug(false).into();
        assert_eq!(m.to_string(), "debug off");
        assert_eq!(gui::Message::from_str("debug blah   off asffd"), Ok(m));
    }

    #[test]
    fn parse_error() {
        assert_matches!(Message::from_str("debug why   \t are you here? 🤨"), Err(MessageParseError::ValueParseError { .. }));
    }
}