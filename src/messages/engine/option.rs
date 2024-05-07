use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use crate::messages::engine::{EngineMessageOptionParameterPointer, EngineMessageParameterPointer, EngineMessagePointer};
use crate::messages::EngineMessage;
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-option-type>
pub enum OptionMessageTypeField {
    /// <https://backscattering.de/chess/uci/#engine-option-type-check>
    Check,
    /// <https://backscattering.de/chess/uci/#engine-option-type-spin>
    Spin,
    /// <https://backscattering.de/chess/uci/#engine-option-type-combo>
    Combo,
    /// <https://backscattering.de/chess/uci/#engine-option-type-button>
    Button,
    /// <https://backscattering.de/chess/uci/#engine-option-type-string>
    String,
}

impl FromStr for OptionMessageTypeField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "check" => Ok(Self::Check),
            "spin" => Ok(Self::Spin),
            "combo" => Ok(Self::Combo),
            "button" => Ok(Self::Button),
            "string" => Ok(Self::String),
            _ => Err(())
        }
    }
}

impl Display for OptionMessageTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Check => f.write_str("check"),
            Self::Spin => f.write_str("spin"),
            Self::Combo => f.write_str("combo"),
            Self::Button => f.write_str("button"),
            Self::String => f.write_str("string"),
        }
    }
}

/// <https://backscattering.de/chess/uci/#engine-option>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionMessage {
    /// <https://backscattering.de/chess/uci/#engine-option-name>
    pub name: String,
    /// <https://backscattering.de/chess/uci/#engine-option-type>
    pub r#type: OptionMessageTypeField,
    /// <https://backscattering.de/chess/uci/#engine-option-default>
    pub default: Option<String>,
    /// <https://backscattering.de/chess/uci/#engine-option-min>
    pub min: Option<isize>,
    /// <https://backscattering.de/chess/uci/#engine-option-max>
    pub max: Option<isize>,
    /// <https://backscattering.de/chess/uci/#engine-option-var>
    pub var: Option<String>,
}

impl TryFrom<RawUciMessage<EngineMessage>> for OptionMessage {
    type Error = MessageTryFromRawUciMessageError<EngineMessageParameterPointer>;

    fn try_from(raw_uci_message: RawUciMessage<EngineMessage>) -> Result<Self, Self::Error> {
        if raw_uci_message.message_pointer != EngineMessagePointer::Option {
            return Err(Self::Error::InvalidMessage);
        };
        
        let Some(name) = raw_uci_message
            .parameters
            .get(&EngineMessageParameterPointer::Option(
                EngineMessageOptionParameterPointer::Name,
            ))
            .cloned()
            else {
                return Err(Self::Error::MissingParameter(
                    EngineMessageParameterPointer::Option(
                        EngineMessageOptionParameterPointer::Name,
                    ),
                ));
            };

        let Some(r#type) = raw_uci_message
            .parameters
            .get(&EngineMessageParameterPointer::Option(
                EngineMessageOptionParameterPointer::Type,
            ))
            .and_then(|s| s.parse().ok())
            else {
                return Err(Self::Error::MissingParameter(
                    EngineMessageParameterPointer::Option(
                        EngineMessageOptionParameterPointer::Type,
                    ),
                ));
            };

        let default = raw_uci_message
            .parameters
            .get(&EngineMessageParameterPointer::Option(
                EngineMessageOptionParameterPointer::Default,
            ))
            .cloned();

        let min = raw_uci_message
            .parameters
            .get(&EngineMessageParameterPointer::Option(
                EngineMessageOptionParameterPointer::Min,
            ))
            .and_then(|s| s.parse().ok());

        let max = raw_uci_message
            .parameters
            .get(&EngineMessageParameterPointer::Option(
                EngineMessageOptionParameterPointer::Max,
            ))
            .and_then(|s| s.parse().ok());

        let var = raw_uci_message
            .parameters
            .get(&EngineMessageParameterPointer::Option(
                EngineMessageOptionParameterPointer::Var,
            ))
            .and_then(|s| s.parse().ok());

        Ok(Self {
            name,
            r#type,
            default,
            min,
            max,
            var,
        })
    }
}

impl Display for OptionMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "option name {} type {}", self.name, self.r#type)?;

        if let Some(default) = &self.default {
            write!(f, " default {default}")?;
        }

        if let Some(min) = self.min {
            write!(f, " min {min}")?;
        }

        if let Some(max) = self.max {
            write!(f, " max {max}")?;
        }

        if let Some(var) = &self.var {
            write!(f, " var {var}")?;
        }

        f.write_char('\n')
    }
}