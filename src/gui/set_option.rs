extern crate alloc;

use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
use core::fmt::{Display, Formatter};
use crate::errors::MessageParseError;
use crate::gui::pointers::{SetOptionParameterPointer};
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::{parsing, MessageParseErrorKind};
use super::{pointers, traits};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Sets a configuration option ([`Option`](crate::engine::Option)) of the engine.
/// They are retrieved with [`Uci`](super::Uci).
/// 
/// <https://backscattering.de/chess/uci/#gui-setoption>
pub struct SetOption<'a> {
    pub name: Cow<'a, str>,
    pub value: Option<Cow<'a, str>>,
}

impl SetOption<'_> {
    /// Calls [`Cow::into_owned`] on each [`Cow`] field.
    /// The resulting value has a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SetOption<'static> {
        SetOption {
            name: Cow::Owned(self.name.into_owned()),
            value: self.value.map(Cow::into_owned).map(Cow::Owned)
        }
    }
}

impl_message!(SetOption<'_>);
message_from_impl!(gui SetOption<'a>);
from_str_parts!(impl SetOption<'_> for parts -> Result {
    let mut name = None;
    let mut value_parameter = None;
    let parameter_fn = |parameter, _, value: &str, parts| {
        match parameter {
            SetOptionParameterPointer::Name => name = Some(value.to_owned()),
            SetOptionParameterPointer::Value => value_parameter = Some(Cow::Owned(value.to_owned())),
        }
        
        Some(parts)
    };
    
    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

    Ok(Self {
        name: Cow::Owned(name.ok_or(MessageParseError {
            expected: "name",
            kind: MessageParseErrorKind::MissingParameters
        })?),
        value: value_parameter,
    })
});

impl Display for SetOption<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.value {
            Some(value) => write!(f, "setoption name {} value {value}", self.name),
            None => write!(f, "setoption name {}", self.name),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use alloc::borrow::Cow;
    use alloc::string::ToString;
    use crate::gui::SetOption;
    use crate::dev_macros::{assert_from_str_message, assert_message_to_from_str, assert_message_to_str};

    #[test]
    fn to_from_str() {
        let m = SetOption {
            name: Cow::from("Skill Level"),
            value: Some(Cow::from("1")),
        };
        
        assert_message_to_from_str!(
            gui 
            m,
            "setoption name Skill Level value 1"
        );

        let m = SetOption {
            name: Cow::from("Skill     Level"),
            value: Some(Cow::from("test   \tfoo")),
        };

        assert_from_str_message!(
            gui
            "setoption value test   \tfoo   \t name     Skill     Level    ",
            Ok(m.clone())
        );
        assert_message_to_str!(
            gui
            m,
            "setoption name Skill     Level value test   \tfoo"
        );
    }
}