use std::fmt::{Display, Formatter, Write};
use crate::messages::gui::{GuiMessageParameterPointer, GuiMessagePointer, GuiMessageSetOptionParameterPointer};
use crate::{MessageTryFromRawMessageError};
use crate::messages::gui::raw_gui_message::RawGuiMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#gui-setoption>
pub struct SetOptionMessage {
    pub name: String,
    pub value: Option<String>,
}

impl TryFrom<RawGuiMessage> for SetOptionMessage {
    type Error = MessageTryFromRawMessageError<GuiMessageParameterPointer>;

    fn try_from(raw_message: RawGuiMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != GuiMessagePointer::SetOption {
            return Err(Self::Error::InvalidMessage);
        };

        let Some(name) = raw_message
            .parameters
            .get(&GuiMessageParameterPointer::SetOption(
                GuiMessageSetOptionParameterPointer::Name,
            ))
            .cloned()
            else {
                return Err(Self::Error::MissingParameter(
                    GuiMessageParameterPointer::SetOption(
                        GuiMessageSetOptionParameterPointer::Name,
                    ),
                ));
            };

        let value = raw_message
            .parameters
            .get(&GuiMessageParameterPointer::SetOption(
                GuiMessageSetOptionParameterPointer::Value,
            ))
            .cloned();

        Ok(Self { name, value })
    }
}

impl Display for SetOptionMessage {
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
    
    use crate::messages::{GuiMessage, SetOptionMessage};
    use pretty_assertions::assert_eq;

    #[test]
    fn to_from_str() {
        let repr = GuiMessage::SetOption(SetOptionMessage {
            name: "Skill Level".to_string(),
            value: Some("1".to_string()),
        });
        let str_repr = "setoption name Skill Level value 1\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(GuiMessage::from_str(str_repr), Ok(repr));
    }
}