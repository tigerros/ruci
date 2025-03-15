extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::from_str_parts::from_str_parts;
use crate::gui::pointers::{SetOptionParameterPointer};
use crate::message_from_impl::message_from_impl;
use crate::parsing;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            Some(value) => write!(f, "setoption name {} value {value}", self.name)?,
            None => write!(f, "setoption name {}", self.name)?,
        }
        
        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use core::str::FromStr;
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;
    use crate::gui::SetOption;
    use crate::Message;

    #[test]
    fn to_from_str() {
        let repr: Message = SetOption {
            name: "Skill Level".to_string(),
            value: Some("1".to_string()),
        }.into();
        let str_repr = "setoption name Skill Level value 1\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }
}