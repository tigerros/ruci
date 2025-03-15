use core::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::from_str_parts::from_str_parts;
use crate::message_from_impl::message_from_impl;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#engine-copyprotection>
pub struct Debug(pub bool);

message_from_impl!(gui Debug);
from_str_parts!(impl Debug for parts -> Result<Self, MessageParseError>  {
    for part in parts {
        match part.trim().to_lowercase().as_str() {
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
            f.write_str("on")?;
        } else {
            f.write_str("off")?;
        }
        
        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    extern crate alloc;
    
    use alloc::string::ToString;
    use core::str::FromStr;
    use crate::Message;
    use super::Debug;
    use crate::MessageParseError;
    use pretty_assertions::{assert_eq, assert_matches};

    #[test]
    fn to_from_str_on() {
        let m: Message = Debug(true).into();
        let str = "debug on\n";
        assert_eq!(m.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), m);
    }

    #[test]
    fn to_from_str_off() {
        let m: Message = Debug(false).into();
        let str = "debug off\n";
        assert_eq!(m.to_string(), str);
        assert_eq!(Message::from_str(str).unwrap(), m);

        let m: Message = Debug(false).into();
        assert_eq!(m.to_string(), "debug off\n");
        assert_eq!(Message::from_str("debug blah   off asffd\n").unwrap(), m);
    }

    #[test]
    fn parse_error() {
        assert_matches!(Message::from_str("debug why   \t are you here? 🤨\n\n"), Err(MessageParseError::ValueParseError { .. }));
    }
}