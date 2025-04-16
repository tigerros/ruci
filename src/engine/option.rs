extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use crate::engine::pointers::OptionParameterPointer;
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, message_from_impl};
use crate::{parsing, OptionReplaceIf};

enum BlankOptionType {
    Check,
    Spin,
    Combo,
    Button,
    String
}

impl FromStr for BlankOptionType {
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

impl Display for BlankOptionType {
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

/// Type of an engine [`Option`].
///
/// <https://backscattering.de/chess/uci/#engine-option-type>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionType<'a> {
    /// <https://backscattering.de/chess/uci/#engine-option-type-check>
    Check {
        /// <https://backscattering.de/chess/uci/#engine-option-default>
        default: StdOption<bool>
    },
    /// <https://backscattering.de/chess/uci/#engine-option-type-spin>
    Spin {
        /// <https://backscattering.de/chess/uci/#engine-option-default>
        default: StdOption<i64>,
        /// <https://backscattering.de/chess/uci/#engine-option-min>
        min: StdOption<i64>,
        /// <https://backscattering.de/chess/uci/#engine-option-max>
        max: StdOption<i64>,
    },
    /// <https://backscattering.de/chess/uci/#engine-option-type-combo>
    Combo {
        /// <https://backscattering.de/chess/uci/#engine-option-default>
        default: StdOption<Cow<'a, str>>,
        /// <https://backscattering.de/chess/uci/#engine-option-var>
        var: Cow<'a, [Cow<'a, str>]>,
    },
    /// <https://backscattering.de/chess/uci/#engine-option-type-button>
    Button,
    /// <https://backscattering.de/chess/uci/#engine-option-type-string>
    String {
        /// <https://backscattering.de/chess/uci/#engine-option-default>
        default: StdOption<Cow<'a, str>>,
    },
}

impl Display for OptionType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("type ")?;

        match self {
            Self::Check { default } => {
                f.write_str("check")?;

                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }
            }
            Self::Spin { default, min, max } => {
                f.write_str("spin")?;

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
            Self::Combo { default, var } => {
                f.write_str("combo")?;

                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }

                for variation in var.iter() {
                    write!(f, " var {variation}")?;
                }
            }
            Self::Button => f.write_str("button")?,
            Self::String { default } => {
                f.write_str("string")?;

                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }
            },
        }

        Ok(())
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
pub struct Option<'a> {
    pub name: Cow<'a, str>,
    pub r#type: OptionType<'a>,
}

message_from_impl!(engine Option<'a>);
from_str_parts!(impl Option<'a> for parts -> Result {
    let mut name = None::<String>;
    let mut r#type = None::<BlankOptionType>;
    let mut default = None::<String>;
    let mut min = None::<i64>;
    let mut max = None::<i64>;
    let mut var = Vec::new();
    let parameter_fn = |parameter, value: &str| match parameter {
        OptionParameterPointer::Name => name = Some(value.to_string()),
        OptionParameterPointer::Type => r#type.replace_if(value.parse().ok()),
        OptionParameterPointer::Default => default = Some(value.to_string()),
        OptionParameterPointer::Min => min.replace_if(value.parse().ok()),
        OptionParameterPointer::Max => max.replace_if(value.parse().ok()),
        OptionParameterPointer::Var => var.push(Cow::Owned(value.to_string())),
    };

    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

    let Some(name) = name else {
        return Err(MessageParseError::MissingParameters { expected: "name" });
    };

    let Some(r#type) = r#type else {
        return Err(MessageParseError::MissingParameters { expected: "a type parameter; check, spin, combo, button or string" });
    };
    
    let name = Cow::Owned(name);

    match r#type {
        BlankOptionType::Check => {
            Ok(Self { name, r#type: OptionType::Check { default: default.and_then(|d| d.parse().ok()) } })
        },
        BlankOptionType::Spin => {
            Ok(Self { name, r#type: OptionType::Spin {
                default: default.and_then(|d| d.parse().ok()),
                min,
                max
            } })
        },
        BlankOptionType::Combo => {
            Ok(Self { name, r#type: OptionType::Combo { default: default.map(Cow::Owned), var: Cow::Owned(var) } })
        },
        BlankOptionType::Button => Ok(Self { name, r#type: OptionType::Button }),
        BlankOptionType::String => {
            Ok(Self { name, r#type: OptionType::String { default: default.map(Cow::Owned) } })
        },
    }
});

impl Display for Option<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "option name {} {}", self.name, self.r#type)
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;
    use alloc::string::ToString;
    use core::str::FromStr;
    use pretty_assertions::assert_eq;
    use crate::{engine, Message};
    use crate::engine::OptionType;
    use super::Option;

    #[test]
    fn to_from_str_min_max() {
        let repr: Message = Option {
            name: Cow::Borrowed("Skill Level"),
            r#type: OptionType::Spin {
                default: Some(20),
                min: Some(-10),
                max: Some(20),
            }
        }.into();

        assert_eq!(repr.to_string(), "option name Skill Level type spin default 20 min -10 max 20");
        assert_eq!(Message::from_str("option name Skill Level type spin type INVALID default 20 min -10 max 20"), Ok(repr));

        let repr: Message = Option {
            name: Cow::Borrowed("Personality"),
            r#type: OptionType::String { default: Some(Cow::Borrowed("Aggressive")) }
        }.into();

        assert_eq!(repr.to_string(), "option name Personality type string default Aggressive");
        assert_eq!(Message::from_str("option name Personality type spin type string default Aggressive"), Ok(repr));
    }

    #[test]
    fn to_from_str_var() {
        let var = &[Cow::Borrowed("Foo bar fighter"), Cow::Borrowed("Aggressive p"), Cow::Borrowed("Defensive p"), Cow::Borrowed("Positional"), Cow::Borrowed("Endgame")];
        
        let repr: engine::Message = Option {
            name: Cow::Borrowed("K Personality"),
            r#type: OptionType::Combo {
                default: Some(Cow::Borrowed("Default p")),
                var: Cow::Borrowed(var),
            }
        }.into();
        let str_in = "option var Foo bar fighter name K Personality type combo default Default p var Aggressive p var Defensive p var Positional var Endgame";
        // Output has a different order, which is fine, but can't use the same string.
        let str_out = "option name K Personality type combo default Default p var Foo bar fighter var Aggressive p var Defensive p var Positional var Endgame";
        assert_eq!(repr.to_string(), str_out);
        assert_eq!(engine::Message::from_str(str_in), Ok(repr));
    }
}