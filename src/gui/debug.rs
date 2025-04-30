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
    use super::Debug;
    use crate::MessageParseError;
    use crate::dev_macros::{assert_from_str_message, assert_message_to_from_str};

    #[test]
    fn to_from_str_on() {
        assert_message_to_from_str!(gui Debug(true), "debug on");
    }

    #[test]
    fn to_from_str_off() {
        assert_message_to_from_str!(gui Debug(false), "debug off");
        assert_from_str_message!(gui "debug blah    off asffd\n", Ok(Debug(false)));
    }

    #[test]
    fn parse_error() {
        assert_from_str_message!(gui "debug why   \t are you \nhere? 🤨", Err::<Debug, MessageParseError>(MessageParseError::ValueParseError {
            expected: "on or off"
        }));
    }
}