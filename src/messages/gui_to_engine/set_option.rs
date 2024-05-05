#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetOptionMessage {
    pub name: String,
    pub value: Option<String>,
}