extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::gui::pointers::RegisterParameterPointer;
use crate::dev_macros::{from_str_parts, message_from_impl};
use crate::parsing;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Tries to register with the engine.
///
/// <https://backscattering.de/chess/uci/#gui-register>
pub enum Register {
    Later,
    Name(String),
    Code(String),
    NameAndCode { name: String, code: String },
}

message_from_impl!(gui Register);
from_str_parts!(impl Register for parts -> Result<Self, MessageParseError>  {
    let mut name = None;
    let mut code = None;
    let parameter_fn = |parameter, value: &str| match parameter {
        RegisterParameterPointer::Name => name = Some(value.to_owned()),
        RegisterParameterPointer::Code => code = Some(value.to_owned()),
    };

    let mut value = String::with_capacity(200);
    let value = parsing::apply_parameters(parts, &mut value, parameter_fn);

    // CLIPPY: It's less readable and also doesn't work
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
    } else if value == "later" {
        Ok(Self::Later)
    } else {
        Err(MessageParseError::MissingParameters { expected: "name, code, or the \"later\" value" })
    }
});

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
    use core::str::FromStr;
    use pretty_assertions::{assert_eq};
    use crate::gui::Register;
    use crate::Message;
    use alloc::string::ToString;

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