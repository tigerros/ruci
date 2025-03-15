use std::fmt::{Display, Formatter, Write};
use crate::engine::pointers::OptionParameterPointer;
use crate::errors::MessageParseError;
use crate::from_str_parts::from_str_parts;
use crate::message_from_impl::message_from_impl;
use crate::parsing;

/// <https://backscattering.de/chess/uci/#engine-option-type>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

from_str_parts!(impl Option for parts {
    let mut name = None::<String>;
    let mut r#type = None::<String>;
    let mut default = None::<String>;
    let mut min = None::<i64>;
    let mut max = None::<i64>;
    let mut variations = Vec::new();
    let mut value = String::with_capacity(50);
    let mut last_parameter = None::<OptionParameterPointer>;
    let mut parameter_to_closure = |parameter, value: &str| match parameter {
        OptionParameterPointer::Name => name = Some(value.to_string()),
        OptionParameterPointer::Type => r#type = Some(value.to_string()),
        OptionParameterPointer::Default => default = Some(value.to_string()),
        OptionParameterPointer::Min => min = value.parse().ok(),
        OptionParameterPointer::Max => max = value.parse().ok(),
        OptionParameterPointer::Var => variations.push(value.to_string()),
    };

    for part in parts {
        let Some(parameter) = parsing::get_parameter_or_update_value(part, &mut value) else {
            continue;
        };

        if let Some(last_parameter) = last_parameter {
            parameter_to_closure(last_parameter, value.trim());
            value.clear();
        }

        last_parameter = Some(parameter);
    }

    if let Some(last_parameter) = last_parameter {
        parameter_to_closure(last_parameter, value.trim());
    }

    let Some(name) = name else {
        return Err(MessageParseError::MissingParameter(OptionParameterPointer::Name.into()));
    };

    let Some(r#type) = r#type else {
        return Err(MessageParseError::MissingParameter(OptionParameterPointer::Type.into()));
    };

    match r#type.as_str() {
        "check" => {
            Ok(Self::Check { name, default: default.and_then(|d| d.parse().ok()) })
        },
        "spin" => {
            Ok(Self::Spin {
                name,
                default: default.and_then(|d| d.parse().ok()),
                min,
                max
            })
        },
        "combo" => {
            Ok(Self::Combo { name, default, variations })
        },
        "button" => Ok(Self::Button { name }),
        "string" => {
            Ok(Self::String { name, default })
        },
        _ => Err(MessageParseError::ParameterParseError(OptionParameterPointer::Type.into())),
    }
});

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
            variations: vec!["Foo bar fighter".to_string(), "Aggressive p".to_string(), "Defensive p".to_string(), "Positional".to_string(), "Endgame".to_string()],
        }.into();
        let str_in = "option var Foo bar fighter name K Personality type combo default Default p var Aggressive p var Defensive p var Positional var Endgame\n";
        // Output has a different order which is fine but can't use the same string
        let str_out = "option name K Personality type combo default Default p var Foo bar fighter var Aggressive p var Defensive p var Positional var Endgame\n";
        assert_eq!(repr.to_string(), str_out);
        assert_eq!(Message::from_str(str_in), Ok(repr));
    }
}