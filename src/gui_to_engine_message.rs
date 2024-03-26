use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroUsize;
use shakmaty::uci::Uci as UciMove;
use crate::join_uci_moves;

pub struct SetOptionMessage {
    pub name: String,
    pub value: Option<String>
}

pub enum SetPositionMessageKind {
    StartingPosition { moves: Option<Vec<UciMove>> },
    Fen { fen: String, moves: Option<Vec<UciMove>> }
}

pub enum RegisterMessageKind {
    Later,
    Name(String),
    Code(String),
    NameAndCode { name: String, code: String }
}

pub struct GoMessage {
    /// https://backscattering.de/chess/uci/#gui-go-searchmoves
    pub search_moves: Option<Vec<UciMove>>,
    /// https://backscattering.de/chess/uci/#gui-go-ponder
    pub ponder: Option<bool>,
    /// https://backscattering.de/chess/uci/#gui-go-wtime
    pub white_time: Option<usize>,
    /// https://backscattering.de/chess/uci/#gui-go-btime
    pub black_time: Option<usize>,
    /// https://backscattering.de/chess/uci/#gui-go-winc
    pub white_increment: Option<NonZeroUsize>,
    /// https://backscattering.de/chess/uci/#gui-go-binc
    pub black_increment: Option<NonZeroUsize>,
    /// https://backscattering.de/chess/uci/#gui-go-movestogo
    pub moves_to_go: Option<NonZeroUsize>,
    /// https://backscattering.de/chess/uci/#gui-go-depth
    pub depth: Option<usize>,
    /// https://backscattering.de/chess/uci/#gui-go-nodes
    pub nodes: Option<usize>,
    /// https://backscattering.de/chess/uci/#gui-go-mate
    pub mate: Option<usize>,
    /// https://backscattering.de/chess/uci/#gui-go-movetime
    pub move_time: Option<usize>,
    /// https://backscattering.de/chess/uci/#gui-go-infinite
    pub infinite: Option<bool>,
}

/// Every message that is sent from the GUI to the engine.
/// Each variant links to a section of the UCI standard where that variant's message is documented.
pub enum GuiToEngineMessage {
    /// https://backscattering.de/chess/uci/#gui-uci
    UseUci,
    /// https://backscattering.de/chess/uci/#gui-debug
    Debug(bool),
    /// https://backscattering.de/chess/uci/#gui-isready
    IsReady,
    /// https://backscattering.de/chess/uci/#gui-setoption
    SetOption(SetOptionMessage),
    /// https://backscattering.de/chess/uci/#gui-register
    Register(RegisterMessageKind),
    /// https://backscattering.de/chess/uci/#gui-ucinewgame
    UciNewGame,
    /// https://backscattering.de/chess/uci/#gui-position
    SetPosition(SetPositionMessageKind),
    /// https://backscattering.de/chess/uci/#gui-go
    Go(GoMessage),
    /// https://backscattering.de/chess/uci/#gui-stop
    Stop,
    /// https://backscattering.de/chess/uci/#gui-ponderhit
    PonderHit,
    /// https://backscattering.de/chess/uci/#gui-quite
    Quit
}

impl Display for GuiToEngineMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseUci => f.write_str("uci"),
            Self::Debug(value) => write!(f, "debug {}", if *value { "on" } else { "off" }),
            Self::IsReady => f.write_str("isready"),
            Self::SetOption(SetOptionMessage { value: None, name }) => write!(f, "setoption name {name}"),
            Self::SetOption(SetOptionMessage { value: Some(value), name }) => write!(f, "setoption name {name} value {value}"),
            Self::Register(RegisterMessageKind::Later) => f.write_str("register later"),
            Self::Register(RegisterMessageKind::Name(name)) => write!(f, "register name {name}"),
            Self::Register(RegisterMessageKind::Code(code)) => write!(f, "register code {code}"),
            Self::Register(RegisterMessageKind::NameAndCode { name, code }) => write!(f, "register name {name} code {code}"),
            Self::UciNewGame => f.write_str("f"),
            Self::SetPosition(SetPositionMessageKind::StartingPosition { moves: None }) => f.write_str("position startpos"),
            Self::SetPosition(SetPositionMessageKind::StartingPosition { moves: Some(moves )}) => write!(f, "position startpos moves {}", join_uci_moves(moves)),
            Self::SetPosition(SetPositionMessageKind::Fen { fen, moves: None }) => write!(f, "position fen {fen}"),
            Self::SetPosition(SetPositionMessageKind::Fen { fen, moves: Some(moves) }) => write!(f, "position fen {fen} moves {}", join_uci_moves(moves)),
            Self::Go(message) => {
                f.write_str("go")?;

                if let Some(search_moves) = message.search_moves {
                    write!(f, " searchmoves {}", join_uci_moves(search_moves))?;
                }

                if let Some(search_moves) = message.search_moves {
                    write!(f, " searchmoves {}", join_uci_moves(search_moves))?;
                }

                if let Some(search_moves) = message.search_moves {
                    write!(f, " searchmoves {}", join_uci_moves(search_moves))?;
                }

                if let Some(search_moves) = message.search_moves {
                    write!(f, " searchmoves {}", join_uci_moves(search_moves))?;
                }

                if let Some(search_moves) = message.search_moves {
                    write!(f, " searchmoves {}", join_uci_moves(search_moves))?;
                }

                if let Some(search_moves) = message.search_moves {
                    write!(f, " searchmoves {}", join_uci_moves(search_moves))?;
                }

                if let Some(search_moves) = message.search_moves {
                    write!(f, " searchmoves {}", join_uci_moves(search_moves))?;
                }
                
                Ok(())
            },
            Self::Stop => f.write_str("stop"),
            Self::PonderHit => f.write_str("ponderhit"),
            Self::Quit => f.write_str("quit")
        }
    }
}