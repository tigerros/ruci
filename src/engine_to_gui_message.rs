use std::str::FromStr;
use shakmaty::uci::Uci as UciMove;

pub struct IdMessage {
    pub name: Option<String>,
    pub author: Option<String>
}

pub struct BestMoveMessage {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>
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

/// <https://backscattering.de/chess/uci/#engine-info-depth>
pub struct InfoMessageDepthField {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: usize,
    /// <https://backscattering.de/chess/uci/#engine-info-seldepth>
    pub selective_search_depth: Option<usize>
}

/// <https://backscattering.de/chess/uci/#engine-info-score>
pub struct InfoMessageScoreField {
    /// <https://backscattering.de/chess/uci/#engine-info-score-cp>
    pub centipawns: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score-mate>
    pub mate_in: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    pub lowerbound: bool,
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    pub upperbound: bool,
}

/// <https://backscattering.de/chess/uci/#engine-info-refutation>
pub struct InfoMessageRefutationField {
    pub refuted_move: UciMove,
    pub refutation: Vec<UciMove>
}

/// <https://backscattering.de/chess/uci/#engine-info-currline>
pub struct InfoMessageCurrentLineField {
    pub used_cpu: Option<usize>,
    pub line: Vec<UciMove>
}

pub struct InfoMessage {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: Option<InfoMessageDepthField>,
    /// <https://backscattering.de/chess/uci/#engine-info-time>
    pub time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-nodes>
    pub nodes: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-pv>
    pub k_best_line: Option<Vec<UciMove>>,
    /// <https://backscattering.de/chess/uci/#engine-info-multipv>
    pub k: Option<usize>,
    //TODO: score (kind of confusing structure) /// <https://backscattering.de/chess/uci/#engine-info-score>
    /// <https://backscattering.de/chess/uci/#engine-info-currmove>
    pub current_move: Option<UciMove>,
    /// <https://backscattering.de/chess/uci/#engine-info-currmovenumber>
    pub current_move_number: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-hashfull>
    pub hash_full: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-nps>
    pub nodes_per_second: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-tbhits>
    pub table_base_hits: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-sbhits>
    pub shredder_base_hits: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-cpuload>
    pub cpu_load: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-string>
    pub string: Option<String>,
    /// <https://backscattering.de/chess/uci/#engine-info-refutation>
    pub refutation: Option<InfoMessageRefutationField>,
    /// <https://backscattering.de/chess/uci/#engine-info-currline>
    pub current_line: Option<InfoMessageCurrentLineField>,
}

pub struct OptionMessage {
    
}

pub enum EngineToGuiMessage {
    /// <https://backscattering.de/chess/uci/#engine-id>
    Id(IdMessage),
    /// <https://backscattering.de/chess/uci/#engine-uciok>
    UciOk,
    /// <https://backscattering.de/chess/uci/#engine-readyok>
    ReadyOk,
    /// <https://backscattering.de/chess/uci/#engine-bestmove>
    BestMove(BestMoveMessage),
    /// <https://backscattering.de/chess/uci/#engine-copyprotection>
    CopyProtection(CopyProtectionMessageKind),
    /// <https://backscattering.de/chess/uci/#engine-registration>
    Registration(RegistrationMessageKind),
    // TODO: Make structs for option messages
    /// <https://backscattering.de/chess/uci/#engine-info>
    Info(Box<InfoMessage>),
    /// <https://backscattering.de/chess/uci/#engine-option>
    Option(String)
}

pub struct EngineToGuiMessages(Vec<EngineToGuiMessage>);

impl FromStr for EngineToGuiMessages {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let mut messages = Vec::with_capacity(parts.len() / 2);

        for part in parts {
            match part {
                "id" => todo!(),
                "uciok" => { messages.push(EngineToGuiMessage::UciOk); },
                "readoky" => { messages.push(EngineToGuiMessage::ReadyOk); },
                _ => todo!()
            }
        }

        Ok(Self(messages))
    }
}

impl FromStr for EngineToGuiMessage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ');


        for part in parts {
            match part {
                "id" => todo!(),
                "uciok" => todo!(),
                _ => todo!()
            }
        }

        todo!()
    }
}