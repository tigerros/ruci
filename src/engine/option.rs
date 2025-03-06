use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;

/// <https://backscattering.de/chess/uci/#engine-option-type>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionType {
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

impl Display for OptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Check => "check",
            Self::Spin => "spin",
            Self::Combo => "combo",
            Self::Button => "button",
            Self::String => "string"
        })
    }
}

type StdOption<T> = std::option::Option<T>;

/// <https://backscattering.de/chess/uci/#engine-option>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Option {
    /// <https://backscattering.de/chess/uci/#engine-option-type-check>
    Check {
        /// <https://backscattering.de/chess/uci/#engine-option-name>
        name: String,
        /// <https://backscattering.de/chess/uci/#engine-option-default>
        default: StdOption<bool>
    },
    /// <https://backscattering.de/chess/uci/#engine-option-type-spin>
    Spin {
        /// <https://backscattering.de/chess/uci/#engine-option-name>
        name: String,
        /// <https://backscattering.de/chess/uci/#engine-option-default>
        default: StdOption<i64>,
        /// <https://backscattering.de/chess/uci/#engine-option-min>
        min: StdOption<i64>,
        /// <https://backscattering.de/chess/uci/#engine-option-max>
        max: StdOption<i64>,
    },
    /// <https://backscattering.de/chess/uci/#engine-option-type-combo>
    Combo {
        /// <https://backscattering.de/chess/uci/#engine-option-name>
        name: String,
        /// <https://backscattering.de/chess/uci/#engine-option-default>
        default: StdOption<String>,
        /// <https://backscattering.de/chess/uci/#engine-option-var>
        variations: Vec<String>,
    },
    /// <https://backscattering.de/chess/uci/#engine-option-type-button>
    Button {
        /// <https://backscattering.de/chess/uci/#engine-option-name>
        name: String,
    },
    /// <https://backscattering.de/chess/uci/#engine-option-type-string>
    String {
        /// <https://backscattering.de/chess/uci/#engine-option-name>
        name: String,
        /// <https://backscattering.de/chess/uci/#engine-option-default>
        default: StdOption<String>,
    },
}

message_from_impl!(engine Option);

impl Option {
    pub const fn name(&self) -> &String {
        match self {
            Self::Check { name, .. } | Self::Spin { name, .. } | Self::Combo { name, .. } | Self::Button { name, .. } | Self::String { name, .. } => name,
        }
    }

    pub const fn r#type(&self) -> OptionType {
        match self {
            Self::Check { .. } => OptionType::Check,
            Self::Spin { .. } => OptionType::Spin,
            Self::Combo { .. } => OptionType::Combo,
            Self::Button { .. } => OptionType::Button,
            Self::String { .. } => OptionType::String
        }
    }
}

impl TryFrom<RawMessage> for Option {
    type Error = MessageParseError;

    fn try_from(raw_message: RawMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != super::pointers::MessagePointer::Option.into() {
            return Err(Self::Error::InvalidMessage);
        };

        let Some(name) = raw_message
            .parameters
            .get(&super::pointers::OptionParameterPointer::Name.into())
            .cloned()
            else {
                return Err(Self::Error::MissingParameter(super::pointers::OptionParameterPointer::Name.into()));
            };

        let Some(r#type) = raw_message
            .parameters
            .get(&super::pointers::OptionParameterPointer::Type.into())
            else {
                return Err(Self::Error::MissingParameter(super::pointers::OptionParameterPointer::Type.into()));
            };

        match r#type.as_bytes() {
            b"check" => {
                let default = raw_message
                    .parameters
                    .get(&super::pointers::OptionParameterPointer::Default.into())
                    .and_then(|s| s.parse().ok());

                Ok(Self::Check { name, default })
            },
            b"spin" => {
                let default = raw_message
                    .parameters
                    .get(&super::pointers::OptionParameterPointer::Default.into())
                    .and_then(|s| s.parse().ok());

                let min = raw_message
                    .parameters
                    .get(&super::pointers::OptionParameterPointer::Min.into())
                    .and_then(|s| s.parse().ok());

                let max = raw_message
                    .parameters
                    .get(&super::pointers::OptionParameterPointer::Max.into())
                    .and_then(|s| s.parse().ok());

                Ok(Self::Spin {
                    name,
                    default,
                    min,
                    max
                })
            },
            b"combo" => {
                let default = raw_message
                    .parameters
                    .get(&super::pointers::OptionParameterPointer::Default.into())
                    .cloned();

                Ok(Self::Combo { name, default, variations: raw_message.option_vars })
            },
            b"button" => Ok(Self::Button { name }),
            b"string" => {
                let default = raw_message
                    .parameters
                    .get(&super::pointers::OptionParameterPointer::Default.into())
                    .cloned();

                Ok(Self::String { name, default })
            },
            _ => Err(MessageParseError::ParameterParseError(super::pointers::OptionParameterPointer::Type.into())),
        }
    }
}

impl Display for Option {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "option name {} type {}", self.name(), self.r#type())?;

        match self {
            Self::Check { default, .. } => if let Some(default) = default {
                write!(f, " default {default}")?;
            }
            Self::Spin { default, min, max, .. } => {
                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }

                if let Some(min) = min {
                    write!(f, " min {min}")?;
                }

                if let Some(max) = max {
                    write!(f, " max {max}")?;
                }
            }
            Self::Combo { default, variations, .. } => {
                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }

                for variation in variations {
                    write!(f, " var {variation}")?;
                }
            }
            Self::Button { .. } => {},
            Self::String { default, .. } => if let Some(default) = default {
                write!(f, " default {default}")?;
            },
        }

        f.write_char('\n')
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use pretty_assertions::assert_eq;
    use crate::Message;
    use super::Option;

    #[test]
    fn to_from_str_min_max() {
        let repr: Message = Option::Spin {
            name: "Skill Level".to_string(),
            default: Some(20),
            min: Some(-10),
            max: Some(20),
        }.into();
        let str_repr = "option name Skill Level type spin default 20 min -10 max 20\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }

    #[test]
    fn to_from_str_var() {
        let repr: Message = Option::Combo {
            name: "K Personality".to_string(),
            default: Some("Default p".to_string()),
            variations: vec!["Aggressive p".to_string(), "Defensive p".to_string(), "Positional".to_string(), "Endgame".to_string()],
        }.into();
        let str_repr = "option name K Personality type combo default Default p var Aggressive p var Defensive p var Positional var Endgame\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }
}