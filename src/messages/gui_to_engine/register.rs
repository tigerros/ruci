#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub enum RegisterMessageKind {
    Later,
    Name(String),
    Code(String),
    NameAndCode { name: String, code: String },
}