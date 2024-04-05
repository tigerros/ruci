#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
/// <https://backscattering.de/chess/uci/#engine-id>
pub enum IdMessageKind {
    /// <https://backscattering.de/chess/uci/#engine-id-name>
    Name(String),
    /// <https://backscattering.de/chess/uci/#engine-id-author>
    Author(String),
    NameAndAuthor(String, String),
}