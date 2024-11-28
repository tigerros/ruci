use std::fmt::{Display, Formatter, Write};
use crate::messages::gui::{GuiMessageParameterPointer, GuiMessagePointer, GuiMessageRegisterParameterPointer};
use crate::{MessageTryFromRawMessageError};
use crate::messages::RawGuiMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#gui-register>
pub enum Register {
    Later,
    Name(String),
    Code(String),
    NameAndCode { name: String, code: String },
}

impl TryFrom<RawGuiMessage> for Register {
    type Error = MessageTryFromRawMessageError<GuiMessageParameterPointer>;

    fn try_from(raw_message: RawGuiMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != GuiMessagePointer::Register {
            return Err(Self::Error::InvalidMessage);
        };

        if let Some(value) = raw_message.value {
            if value == "later" {
                return Ok(Self::Later);
            }
        }

        let name = raw_message
            .parameters
            .get(&GuiMessageParameterPointer::Register(
                GuiMessageRegisterParameterPointer::Name,
            ))
            .cloned();

        let code = raw_message
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

impl Display for Register {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use pretty_assertions::assert_eq;
    
    use crate::messages::{GuiMessage, Register};

    #[test]
    fn to_from_str() {
        let repr = GuiMessage::Register(Register::NameAndCode {
            name: "john smith".to_string(),
            code: "31 tango".to_string()
        });
        let str_repr = "register name john smith code 31 tango\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(GuiMessage::from_str(str_repr), Ok(repr));
    }
}