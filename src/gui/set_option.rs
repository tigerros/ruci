extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::{Display, Formatter};
use crate::errors::MessageParseError;
use crate::gui::pointers::{SetOptionParameterPointer};
use crate::dev_macros::{from_str_parts, message_from_impl};
use crate::parsing;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Sets a configuration option ([`Option`](crate::engine::Option)) of the engine.
/// They are retrieved with [`Uci`](super::Uci).
/// 
/// <https://backscattering.de/chess/uci/#gui-setoption>
pub struct SetOption {
    pub name: String,
    pub value: Option<String>,
}

message_from_impl!(gui SetOption);
from_str_parts!(impl SetOption for parts -> Result<Self, MessageParseError> {
    let mut name = None;
    let mut value_parameter = None;
    let parameter_fn = |parameter: SetOptionParameterPointer, value: &str| match parameter {
        SetOptionParameterPointer::Name => name = Some(value.to_owned()),
        SetOptionParameterPointer::Value => value_parameter = Some(value.to_owned()),
    };
    
    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

    Ok(Self {
        name: name.ok_or(MessageParseError::MissingParameters { expected: "name" })?,
        value: value_parameter,
    })
});

impl Display for SetOption {
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
    use core::str::FromStr;
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;
    use crate::gui::SetOption;
    use crate::{gui, Message};

    #[test]
    fn to_from_str() {
        let repr: Message = SetOption {
            name: "Skill Level".to_string(),
            value: Some("1".to_string()),
        }.into();
        let str_repr = "setoption name Skill Level value 1";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));

        let repr: gui::Message = SetOption {
            name: "Skill     Level".to_string(),
            value: Some("test   \tfoo".to_string()),
        }.into();

        assert_eq!(repr.to_string(), "setoption name Skill     Level value test   \tfoo");
        assert_eq!(gui::Message::from_str("setoption value test   \tfoo   \t name     Skill     Level    "), Ok(repr));
    }
}