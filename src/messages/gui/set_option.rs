use std::fmt::{Display, Formatter, Write};
use crate::messages::{GuiMessage};
use crate::messages::gui::{GuiMessageParameterPointer, GuiMessagePointer, GuiMessageSetOptionParameterPointer};
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#gui-setoption>
pub struct SetOptionMessage {
    pub name: String,
    pub value: Option<String>,
}

impl TryFrom<RawUciMessage<GuiMessage>> for SetOptionMessage {
    type Error = MessageTryFromRawUciMessageError<GuiMessageParameterPointer>;

    fn try_from(raw_uci_message: RawUciMessage<GuiMessage>) -> Result<Self, Self::Error> {
        if raw_uci_message.message_pointer != GuiMessagePointer::SetOption {
            return Err(Self::Error::InvalidMessage);
        };

        let Some(name) = raw_uci_message
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

        let value = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::SetOption(
                GuiMessageSetOptionParameterPointer::Name,
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