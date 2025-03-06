use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// <https://backscattering.de/chess/uci/#gui-setoption>
pub struct SetOption {
    pub name: String,
    pub value: Option<String>,
}

message_from_impl!(gui SetOption);

impl TryFrom<RawMessage> for SetOption {
    type Error = MessageParseError;

    fn try_from(mut raw_message: RawMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != super::pointers::MessagePointer::SetOption.into() {
            return Err(Self::Error::InvalidMessage);
        };

        let Some(name) = raw_message
            .parameters
            .remove(&super::pointers::SetOptionParameterPointer::Name.into())
            else {
                return Err(Self::Error::MissingParameter(super::pointers::SetOptionParameterPointer::Name.into()));
            };

        let value = raw_message
            .parameters
            .remove(&super::pointers::SetOptionParameterPointer::Value.into());

        Ok(Self { name, value })
    }
}

impl Display for SetOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    use std::str::FromStr;
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