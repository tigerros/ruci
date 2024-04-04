use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
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