extern crate alloc;

use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
use core::fmt::{Display, Formatter};
use crate::errors::MessageParseError;
use crate::gui::pointers::RegisterParameterPointer;
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::{parsing, MessageParseErrorKind};
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

impl Register<'_> {
    /// Calls [`Cow::into_owned`] on each [`Cow`] field.
    /// The resulting value has a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Register<'static> {
        match self {
            Self::Later => Register::Later,
            Self::Name(name) => Register::Name(Cow::Owned(name.into_owned())),
            Self::Code(code) => Register::Code(Cow::Owned(code.into_owned())),
            Self::NameAndCode { name, code } => Register::NameAndCode {
                name: Cow::Owned(name.into_owned()),
                code: Cow::Owned(code.into_owned()),
            }
        }
    }
}

impl_message!(Register<'_>);
message_from_impl!(gui Register<'a>);
from_str_parts!(impl Register<'_> for parts -> Result {
    let mut name = None;
    let mut code = None;
    let parameter_fn = |parameter, _, value: &str, parts| {
        match parameter {
            RegisterParameterPointer::Name => name = Some(value.to_owned()),
            RegisterParameterPointer::Code => code = Some(value.to_owned()),
        }
        
        Some(parts)
    };

    let mut value = String::with_capacity(200);
    // CLIPPY: closure always returns Some so the final value will be Some too
    #[allow(clippy::unwrap_used)]
    let value = parsing::apply_parameters(parts, &mut value, parameter_fn).unwrap();

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
        Err(MessageParseError {
            expected: "name, code, or the \"later\" value",
            kind: MessageParseErrorKind::MissingParameters
        })
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
    use crate::gui::Register;
    use alloc::string::ToString;
    use crate::dev_macros::{assert_from_str_message, assert_message_to_from_str, assert_message_to_str};

    #[test]
    fn to_from_str() {
        let m = Register::NameAndCode {
            name: Cow::Borrowed("john later smith"),
            code: Cow::Borrowed("31 tango")
        };
        
        assert_message_to_from_str!(
            gui 
            m,
            "register name john later smith code 31 tango"
        );
    }

    #[test]
    fn to_from_str_later() {
        let m = Register::Later;

        assert_from_str_message!(
            gui
            "register   later   ff\n\n",
            Ok(m.clone())
        );
        assert_message_to_str!(
            gui
            m,
            "register later"
        );
    }

    #[test]
    fn invalid_parameter() {
        let m = Register::Name(Cow::Borrowed("a l o t o f s p a c e s"));

        assert_from_str_message!(
            gui
            "register blahblah woo name a l o t o f s p a c e s\n",
            Ok(m.clone())
        );
        assert_message_to_str!(
            gui
            m,
            "register name a l o t o f s p a c e s"
        );
    }
}