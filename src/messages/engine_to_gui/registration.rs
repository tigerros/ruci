use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum RegistrationMessageKind {
    Checking,
    Ok,
    Error,
}

impl FromStr for RegistrationMessageKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "checking" => Ok(Self::Checking),
            "ok" => Ok(Self::Ok),
            "error" => Ok(Self::Error),
            _ => Err(()),
        }
    }
}

impl Display for RegistrationMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Checking => f.write_str("checking"),
            Self::Ok => f.write_str("ok"),
            Self::Error => f.write_str("error"),
        }
    }
}