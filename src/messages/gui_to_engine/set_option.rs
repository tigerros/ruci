#[derive(Debug, Clone)]
pub struct SetOptionMessage {
    pub name: String,
    pub value: Option<String>,
}