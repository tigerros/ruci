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