
#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
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