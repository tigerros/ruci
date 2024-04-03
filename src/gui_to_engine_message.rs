use std::collections::HashMap;
use crate::{join_uci_moves, MessageParameterPointer, MessageParameterPointerParseError, MessagePointer, ParameterValue, RawUciMessage};
use crate::UciMoveList;
use std::fmt::{Debug, Display, Formatter};
use std::num::NonZeroUsize;
use crate::define_message_enum::define_message_enum;
use crate::traits::Message;

pub struct SetOptionMessage {
    pub name: String,
    pub value: Option<String>,
}

pub enum SetPositionMessageKind {
    StartingPosition {
        moves: Option<UciMoveList>,
    },
    Fen {
        fen: String,
        moves: Option<UciMoveList>,
    },
}

pub enum RegisterMessageKind {
    Later,
    Name(String),
    Code(String),
    NameAndCode { name: String, code: String },
}

pub struct GoMessage {
    /// <https://backscattering.de/chess/uci/#gui-go-searchmoves>
    pub search_moves: Option<UciMoveList>,
    /// <https://backscattering.de/chess/uci/#gui-go-ponder>
    pub ponder: bool,
    /// <https://backscattering.de/chess/uci/#gui-go-wtime>
    pub white_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-btime>
    pub black_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-winc>
    pub white_increment: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-binc>
    pub black_increment: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-movestogo>
    pub moves_to_go: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-depth>
    pub depth: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-nodes>
    pub nodes: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-mate>
    pub mate: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-movetime>
    pub move_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-infinite>
    pub infinite: bool,
}

define_message_enum! {
    /// Every message that is sent from the GUI to the engine.
    /// Each variant links to a section of the UCI standard where that variant's message is documented.
    pub enum GuiToEngineMessage {
        /// <https://backscattering.de/chess/uci/#gui-uci>
        %["uci"]
        UseUci,
        /// <https://backscattering.de/chess/uci/#gui-debug>
        %["debug"]
        Debug(%bool),
        /// <https://backscattering.de/chess/uci/#gui-isready>
        %["isready"]
        IsReady,
        /// <https://backscattering.de/chess/uci/#gui-setoption>
        %["setoption"]
        %%[parameters = [(Name, "name"), (Value, "value")]]
        SetOption(%SetOptionMessage),
        /// <https://backscattering.de/chess/uci/#gui-register>
        %["register"]
        %%[parameters = [(Name, "name"), (Code, "code")]]
        Register(%RegisterMessageKind),
        /// <https://backscattering.de/chess/uci/#gui-ucinewgame>
        %["ucinewgame"]
        UciNewGame,
        /// <https://backscattering.de/chess/uci/#gui-position>
        %["setposition"]
        %%[parameters = [(Fen, "fen"), (Moves, "moves")]]
        SetPosition(%SetPositionMessageKind),
        /// <https://backscattering.de/chess/uci/#gui-go>
        %["go"]
        %%[parameters = [(SearchMoves, "searchmoves"), **(Ponder, "ponder"), (WhiteTime, "wtime"), (BlackTime, "btime"), (WhiteIncrement, "winc"), (BlackIncrement, "binc"), (MovesToGo, "movestogo"), (Depth, "depth"), (Nodes, "nodes"), (Mate, "mate"), (MoveTime, "movetime"), **(Infinite, "infinite")]]
        Go(%GoMessage),
        /// <https://backscattering.de/chess/uci/#gui-stop>
        %["stop"]
        Stop,
        /// <https://backscattering.de/chess/uci/#gui-ponderhit>
        %["ponderhit"]
        PonderHit,
        /// <https://backscattering.de/chess/uci/#gui-quite>
        %["quit"]
        Quit
    }
}

impl Display for GuiToEngineMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseUci => f.write_str("uci"),
            Self::Debug(value) => write!(f, "debug {}", if *value { "on" } else { "off" }),
            Self::IsReady => f.write_str("isready"),
            Self::SetOption(SetOptionMessage { value: None, name }) => {
                write!(f, "setoption name {name}")
            }
            Self::SetOption(SetOptionMessage {
                value: Some(value),
                name,
            }) => write!(f, "setoption name {name} value {value}"),
            Self::Register(RegisterMessageKind::Later) => f.write_str("register later"),
            Self::Register(RegisterMessageKind::Name(name)) => write!(f, "register name {name}"),
            Self::Register(RegisterMessageKind::Code(code)) => write!(f, "register code {code}"),
            Self::Register(RegisterMessageKind::NameAndCode { name, code }) => {
                write!(f, "register name {name} code {code}")
            }
            Self::UciNewGame => f.write_str("f"),
            Self::SetPosition(SetPositionMessageKind::StartingPosition { moves: None }) => {
                f.write_str("position startpos")
            }
            Self::SetPosition(SetPositionMessageKind::StartingPosition { moves: Some(moves) }) => {
                write!(f, "position startpos moves {}", join_uci_moves(&moves.0))
            }
            Self::SetPosition(SetPositionMessageKind::Fen { fen, moves: None }) => {
                write!(f, "position fen {fen}")
            }
            Self::SetPosition(SetPositionMessageKind::Fen {
                fen,
                moves: Some(moves),
            }) => write!(f, "position fen {fen} moves {}", join_uci_moves(&moves.0)),
            Self::Go(message) => {
                f.write_str("go")?;

                if let Some(search_moves) = &message.search_moves {
                    write!(f, " searchmoves {}", join_uci_moves(&search_moves.0))?;
                }

                if message.ponder {
                    f.write_str(" ponder")?;
                }

                if let Some(white_time) = message.white_time {
                    write!(f, " wtime {white_time}")?;
                }

                if let Some(black_time) = message.black_time {
                    write!(f, " btime {black_time}")?;
                }

                if let Some(white_increment) = message.white_increment {
                    write!(f, " winc {white_increment}")?;
                }

                if let Some(black_increment) = message.black_increment {
                    write!(f, " binc {black_increment}")?;
                }

                if let Some(moves_to_go) = message.moves_to_go {
                    write!(f, " moves_to_go {moves_to_go}")?;
                }

                if let Some(depth) = message.depth {
                    write!(f, " depth {depth}")?;
                }

                if let Some(nodes) = message.nodes {
                    write!(f, " nodes {nodes}")?;
                }

                if let Some(mate) = message.mate {
                    write!(f, " mate {mate}")?;
                }

                if let Some(move_time) = message.move_time {
                    write!(f, " movetime {move_time}")?;
                }

                if message.infinite {
                    f.write_str(" infinite")?;
                }

                Ok(())
            }
            Self::Stop => f.write_str("stop"),
            Self::PonderHit => f.write_str("ponderhit"),
            Self::Quit => f.write_str("quit"),
        }
    }
}
