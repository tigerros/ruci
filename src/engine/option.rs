extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use crate::engine::pointers::OptionParameterPointer;
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, message_from_impl};
use crate::{parsing, OptionReplaceIf};

/// Engine option type.
///
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

impl FromStr for OptionType {
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

impl Display for OptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Check => "check",
            Self::Spin => "spin",
            Self::Combo => "combo",
            Self::Button => "button",
            Self::String => "string"
        })
    }
}

type StdOption<T> = core::option::Option<T>;

/// Engine configuration option.
///
/// Sent after [`Uci`](crate::gui::Uci).
///
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

from_str_parts!(impl Option for parts -> Result<Self, MessageParseError>  {
    let mut name = None::<String>;
    let mut r#type = None::<OptionType>;
    let mut default = None::<String>;
    let mut min = None::<i64>;
    let mut max = None::<i64>;
    let mut variations = Vec::new();
    let parameter_fn = |parameter, value: &str| match parameter {
        OptionParameterPointer::Name => name = Some(value.to_string()),
        OptionParameterPointer::Type => r#type.replace_if(value.parse().ok()),
        OptionParameterPointer::Default => default = Some(value.to_string()),
        OptionParameterPointer::Min => min.replace_if(value.parse().ok()),
        OptionParameterPointer::Max => max.replace_if(value.parse().ok()),
        OptionParameterPointer::Var => variations.push(value.to_string()),
    };

    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

    let Some(name) = name else {
        return Err(MessageParseError::MissingParameters { expected: "name" });
    };

    let Some(r#type) = r#type else {
        return Err(MessageParseError::MissingParameters { expected: "a type parameter; check, spin, combo, button or string" });
    };

    match r#type {
        OptionType::Check => {
            Ok(Self::Check { name, default: default.and_then(|d| d.parse().ok()) })
        },
        OptionType::Spin => {
            Ok(Self::Spin {
                name,
                default: default.and_then(|d| d.parse().ok()),
                min,
                max
            })
        },
        OptionType::Combo => {
            Ok(Self::Combo { name, default, variations })
        },
        OptionType::Button => Ok(Self::Button { name }),
        OptionType::String => {
            Ok(Self::String { name, default })
        },
    }
});

impl Display for Option {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;
    use core::str::FromStr;
    use pretty_assertions::assert_eq;
    use crate::{engine, Message};
    use super::Option;

    #[test]
    fn to_from_str_min_max() {
        let repr: Message = Option::Spin {
            name: "Skill Level".to_string(),
            default: Some(20),
            min: Some(-10),
            max: Some(20),
        }.into();

        assert_eq!(repr.to_string(), "option name Skill Level type spin default 20 min -10 max 20");
        assert_eq!(Message::from_str("option name Skill Level type spin type INVALID default 20 min -10 max 20"), Ok(repr));

        let repr: Message = Option::Spin {
            name: "Skill Level".to_string(),
            default: Some(20),
            min: Some(-10),
            max: Some(20),
        }.into();

        assert_eq!(repr.to_string(), "option name Skill Level type spin default 20 min -10 max 20");
        assert_eq!(Message::from_str("option name Skill Level type spin default lol default 20 min -10 max 20 max usetheonebefore"), Ok(repr));
    }

    #[test]
    fn to_from_str_var() {
        let repr: engine::Message = Option::Combo {
            name: "K Personality".to_string(),
            default: Some("Default p".to_string()),
            variations: vec!["Foo bar fighter".to_string(), "Aggressive p".to_string(), "Defensive p".to_string(), "Positional".to_string(), "Endgame".to_string()],
        }.into();
        let str_in = "option var Foo bar fighter name K Personality type combo default Default p var Aggressive p var Defensive p var Positional var Endgame";
        // Output has a different order which is fine but can't use the same string
        let str_out = "option name K Personality type combo default Default p var Foo bar fighter var Aggressive p var Defensive p var Positional var Endgame";
        assert_eq!(repr.to_string(), str_out);
        assert_eq!(engine::Message::from_str(str_in), Ok(repr));
    }
}