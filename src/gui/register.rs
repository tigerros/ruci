extern crate alloc;

use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
use core::fmt::{Display, Formatter};
use crate::errors::MessageParseError;
use crate::gui::pointers::RegisterParameterPointer;
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::parsing;
use super::{pointers, traits};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Tries to register with the engine.
///
/// <https://backscattering.de/chess/uci/#gui-register>
pub enum Register<'a> {
    Later,
    Name(Cow<'a, str>),
    Code(Cow<'a, str>),
    NameAndCode { name: Cow<'a, str>, code: Cow<'a, str> },
}

impl_message!(Register<'_>);
message_from_impl!(gui Register<'a>);
from_str_parts!(impl Register<'a> for parts -> Result {
    let mut name = None;
    let mut code = None;
    let parameter_fn = |parameter, value: &str| match parameter {
        RegisterParameterPointer::Name => name = Some(value.to_owned()),
        RegisterParameterPointer::Code => code = Some(value.to_owned()),
    };

    let mut value = String::with_capacity(200);
    let value = parsing::apply_parameters(parts, &mut value, parameter_fn);

    // CLIPPY: It is less readable and doesn't work
    #[allow(clippy::option_if_let_else)]
    if let Some(name) = name {
        if let Some(code) = code {
            Ok(Self::NameAndCode {
                name: Cow::Owned(name),
                code: Cow::Owned(code),
            })
        } else {
            Ok(Self::Name(Cow::Owned(name)))
        }
    } else if let Some(code) = code {
        Ok(Self::Code(Cow::Owned(code)))
    } else if value.split(' ').any(|s| s == "later") {
        Ok(Self::Later)
    } else {
        Err(MessageParseError::MissingParameters { expected: "name, code, or the \"later\" value" })
    }
});

impl Display for Register<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Later => f.write_str("register later"),
            Self::Name(name) => write!(f, "register name {name}"),
            Self::Code(code) => write!(f, "register code {code}"),
            Self::NameAndCode { name, code } => {
                write!(f, "register name {name} code {code}")
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use alloc::borrow::Cow;
    use core::str::FromStr;
    use pretty_assertions::{assert_eq};
    use crate::gui::Register;
    use crate::{gui, Message};
    use alloc::string::ToString;

    #[test]
    fn to_from_str() {
        let m: Message = Register::NameAndCode {
            name: Cow::Borrowed("john smith"),
            code: Cow::Borrowed("31 tango")
        }.into();
        let str = "register name john smith code 31 tango";

        assert_eq!(m.to_string(), str);
        assert_eq!(Message::from_str(str), Ok(m));
    }

    #[test]
    fn to_from_str_later() {
        let m: Message = Register::Later.into();

        assert_eq!(m.to_string(), "register later");
        assert_eq!(Message::from_str("register   later   ff\n\n"), Ok(m));
    }

    #[test]
    fn invalid_parameter() {
        let m: gui::Message = Register::Name(Cow::Borrowed("a l o t o f s p a c e s")).into();
        assert_eq!(m.to_string(), "register name a l o t o f s p a c e s");
        assert_eq!(gui::Message::from_str("register blahblah woo name a l o t o f s p a c e s\n").unwrap(), m);
    }
}