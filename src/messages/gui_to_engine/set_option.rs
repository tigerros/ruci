#[derive(Debug)]
pub struct SetOptionMessage {
    pub name: String,
    pub value: Option<String>,
}