#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#gui-setoption>
pub struct SetOptionMessage {
    pub name: String,
    pub value: Option<String>,
}