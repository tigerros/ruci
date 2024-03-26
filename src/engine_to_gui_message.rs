pub struct IdMessage {
    pub name: Option<String>,
    pub author: Option<String>
}

pub enum CopyProtectionMessageKind {
    Ok,
    Error
}

pub enum RegistrationMessageKind {
    Checking,
    Ok,
    Error
}

pub enum EngineToGuiMessage {
    /// https://backscattering.de/chess/uci/#engine-id
    Id(IdMessage),
    /// https://backscattering.de/chess/uci/#engine-uciok
    UciOk,
    /// https://backscattering.de/chess/uci/#engine-readyok
    ReadyOk,
    /// https://backscattering.de/chess/uci/#engine-bestmove
    BestMove,
    /// https://backscattering.de/chess/uci/#engine-copyprotection
    CopyProtection(CopyProtectionMessageKind),
    /// https://backscattering.de/chess/uci/#engine-registration
    Registration(RegistrationMessageKind),
    // TODO: Make structs for info and option messages
    /// https://backscattering.de/chess/uci/#engine-info
    Info(String),
    /// https://backscattering.de/chess/uci/#engine-option
    Option(String)
}