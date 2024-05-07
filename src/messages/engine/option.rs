use std::fmt::{Display, Formatter};
use std::str::FromStr;

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