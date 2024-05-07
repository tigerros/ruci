use std::fmt::{Display, Formatter, Write};
use crate::messages::{GuiMessage, SetPositionMessageKind};
use crate::messages::gui::{GuiMessageParameterPointer, GuiMessagePointer, GuiMessageRegisterParameterPointer};
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#gui-register>
pub enum RegisterMessageKind {
    Later,
    Name(String),
    Code(String),
    NameAndCode { name: String, code: String },
}

impl TryFrom<RawUciMessage<GuiMessage>> for RegisterMessageKind {
    type Error = MessageTryFromRawUciMessageError<GuiMessageParameterPointer>;

    fn try_from(raw_uci_message: RawUciMessage<GuiMessage>) -> Result<Self, Self::Error> {
        if raw_uci_message.message_pointer != GuiMessagePointer::Register {
            return Err(Self::Error::InvalidMessage);
        };

        if let Some(value) = raw_uci_message.value {
            if value == "later" {
                return Ok(Self::Later);
            }
        }

        let name = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Register(
                GuiMessageRegisterParameterPointer::Name,
            ))
            .cloned();

        let code = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Register(
                GuiMessageRegisterParameterPointer::Code,
            ))
            .cloned();

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
            Err(Self::Error::MissingParameter(
                GuiMessageParameterPointer::Register(
                    GuiMessageRegisterParameterPointer::Name,
                ),
            ))
        }
    }
}

impl Display for RegisterMessageKind {
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