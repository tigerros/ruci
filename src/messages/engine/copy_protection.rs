use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-copyprotection>
pub enum CopyProtectionMessageKind {
    Ok,
    Error,
}

impl FromStr for CopyProtectionMessageKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ok" => Ok(Self::Ok),
            "error" => Ok(Self::Error),
            _ => Err(()),
        }
    }
}

impl Display for CopyProtectionMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("ok"),
            Self::Error => f.write_str("error"),
        }
    }
}