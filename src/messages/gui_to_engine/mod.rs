dry_mods::mods! {
    mod pub use go, register, set_option, set_position;
}

use crate::define_message_enum::define_message_enum;
use crate::RawUciMessage;
use std::fmt::{Debug, Display, Formatter};

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
        /// <https://backscattering.de/chess/uci/#gui-quit>
        %["quit"]
        Quit
    }
}

impl TryFrom<RawUciMessage<GuiToEngineMessagePointer, GuiToEngineMessageParameterPointer>>
    for GuiToEngineMessage
{
    type Error = ();

    fn try_from(
        raw_uci_message: RawUciMessage<
            GuiToEngineMessagePointer,
            GuiToEngineMessageParameterPointer,
        >,
    ) -> Result<Self, Self::Error> {
        // Parameter-less value-less messages
        match raw_uci_message.message_pointer {
            GuiToEngineMessagePointer::UseUci => return Ok(Self::UseUci),
            GuiToEngineMessagePointer::IsReady => return Ok(Self::IsReady),
            GuiToEngineMessagePointer::UciNewGame => return Ok(Self::UciNewGame),
            GuiToEngineMessagePointer::Stop => return Ok(Self::Stop),
            GuiToEngineMessagePointer::PonderHit => return Ok(Self::PonderHit),
            GuiToEngineMessagePointer::Quit => return Ok(Self::Quit),
            _ => {}
        }

        // Value-less messages
        match raw_uci_message.message_pointer {
            GuiToEngineMessagePointer::SetOption => {
                let Some(name) = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::SetOption(
                        GuiToEngineMessageSetOptionParameterPointer::Name,
                    ))
                    .and_then(|p| p.some())
                    .cloned()
                else {
                    return Err(());
                };

                let value = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::SetOption(
                        GuiToEngineMessageSetOptionParameterPointer::Name,
                    ))
                    .and_then(|p| p.some())
                    .cloned();

                return Ok(Self::SetOption(SetOptionMessage { name, value }));
            }
            GuiToEngineMessagePointer::Register => {
                let name = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Register(
                        GuiToEngineMessageRegisterParameterPointer::Name,
                    ))
                    .and_then(|p| p.some())
                    .cloned();

                let code = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Register(
                        GuiToEngineMessageRegisterParameterPointer::Code,
                    ))
                    .and_then(|p| p.some())
                    .cloned();

                if let Some(name) = name {
                    if let Some(code) = code {
                        return Ok(Self::Register(RegisterMessageKind::NameAndCode {
                            name,
                            code,
                        }));
                    }

                    return Ok(Self::Register(RegisterMessageKind::Name(name)));
                } else if let Some(code) = code {
                    return Ok(Self::Register(RegisterMessageKind::Code(code)));
                }
            }
            GuiToEngineMessagePointer::SetPosition => {
                let fen = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::SetPosition(
                        GuiToEngineMessageSetPositionParameterPointer::Fen,
                    ))
                    .and_then(|p| p.some())
                    .cloned();

                let moves = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::SetPosition(
                        GuiToEngineMessageSetPositionParameterPointer::Moves,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                if let Some(fen) = fen {
                    return Ok(Self::SetPosition(SetPositionMessageKind::Fen {
                        fen,
                        moves,
                    }));
                }

                return Ok(Self::SetPosition(
                    SetPositionMessageKind::StartingPosition { moves },
                ));
            }
            GuiToEngineMessagePointer::Go => {
                let search_moves = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::SearchMoves,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let ponder = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::Ponder,
                    ))
                    .is_some();

                let white_time = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::WhiteTime,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let black_time = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::BlackTime,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let white_increment = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::WhiteIncrement,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let black_increment = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::BlackIncrement,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let moves_to_go = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::MovesToGo,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let depth = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::Depth,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let nodes = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::Nodes,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let mate = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::Mate,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let move_time = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::MoveTime,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let infinite = raw_uci_message
                    .parameters
                    .get(&GuiToEngineMessageParameterPointer::Go(
                        GuiToEngineMessageGoParameterPointer::Infinite,
                    ))
                    .is_some();

                return Ok(Self::Go(GoMessage {
                    search_moves,
                    ponder,
                    white_time,
                    black_time,
                    white_increment,
                    black_increment,
                    moves_to_go,
                    depth,
                    nodes,
                    mate,
                    move_time,
                    infinite,
                }));
            }
            _ => {}
        }

        let Some(value) = raw_uci_message.value else {
            return Err(());
        };

        // Messages with parameters/values
        match raw_uci_message.message_pointer {
            GuiToEngineMessagePointer::Debug => match value.as_bytes() {
                b"on" => return Ok(Self::Debug(true)),
                b"off" => return Ok(Self::Debug(false)),
                _ => {}
            },
            GuiToEngineMessagePointer::Register => match value.as_bytes() {
                b"later" => return Ok(Self::Register(RegisterMessageKind::Later)),
                _ => {}
            },
            _ => {}
        }

        todo!()
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
                write!(f, "position startpos moves {}", &moves)
            }
            Self::SetPosition(SetPositionMessageKind::Fen { fen, moves: None }) => {
                write!(f, "position fen {fen}")
            }
            Self::SetPosition(SetPositionMessageKind::Fen {
                fen,
                moves: Some(moves),
            }) => write!(f, "position fen {fen} moves {}", &moves),
            Self::Go(message) => {
                f.write_str("go")?;

                if let Some(search_moves) = &message.search_moves {
                    write!(f, " searchmoves {}", &search_moves)?;
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