use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// <https://backscattering.de/chess/uci/#gui-register>
pub enum Register {
    Later,
    Name(String),
    Code(String),
    NameAndCode { name: String, code: String },
}

message_from_impl!(gui Register);

impl TryFrom<RawMessage> for Register {
    type Error = MessageParseError;

    fn try_from(mut raw_message: RawMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != super::pointers::MessagePointer::Register.into() {
            return Err(Self::Error::InvalidMessage);
        };

        if let Some(value) = raw_message.value {
            if value == "later" {
                return Ok(Self::Later);
            }
        }

        let name = raw_message
            .parameters
            .remove(&super::pointers::RegisterParameterPointer::Name.into());

        let code = raw_message
            .parameters
            .remove(&super::pointers::RegisterParameterPointer::Code.into());

        #[allow(clippy::option_if_let_else)]
        if let Some(name) = name {
            if let Some(code) = code {
                Ok(Self::NameAndCode {
                    name,
                    code,
                })
            } else {
                Ok(Self::Name(name))
            }
        } else if let Some(code) = code {
            Ok(Self::Code(code))
        } else {
            Err(Self::Error::MissingParameter(super::pointers::RegisterParameterPointer::Name.into()))
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Later => f.write_str("register later")?,
            Self::Name(name) => write!(f, "register name {name}")?,
            Self::Code(code) => write!(f, "register code {code}")?,
            Self::NameAndCode { name, code } => {
                write!(f, "register name {name} code {code}")?;
            }
        }
        
        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::str::FromStr;
    use pretty_assertions::assert_eq;
    use crate::gui::Register;
    use crate::Message;

    #[test]
    fn to_from_str() {
        let m: Message = Register::NameAndCode {
            name: "john smith".to_string(),
            code: "31 tango".to_string()
        }.into();
        let str = "register name john smith code 31 tango\n";

        assert_eq!(m.to_string(), str);
        assert_eq!(Message::from_str(str), Ok(m));
    }

    #[test]
    fn invalid_parameter() {
        let m: Message = Register::Name("a l o t o f s p a c e s".to_string()).into();
        assert_eq!(m.to_string(), "register name a l o t o f s p a c e s\n");
        assert_eq!(Message::from_str("register blahblah woo name a l o t o f s p a c e s\n").unwrap(), m);
    }
}