dry_mods::mods! {
    mod pub use go, register, set_option, set_position;
}

// TODO: Tests

use crate::define_message_enum::define_message_enum;
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};
use std::fmt::{Debug, Display, Formatter, Write};

define_message_enum! {
    /// Every message that is sent from the GUI to the engine.
    /// Each variant links to a section of the UCI standard where that variant's message is documented.
    pub enum GuiMessage {
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
        %["position"]
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

impl TryFrom<RawUciMessage<GuiMessagePointer, GuiMessageParameterPointer>> for GuiMessage {
    type Error = MessageTryFromRawUciMessageError<GuiMessageParameterPointer>;

    #[allow(clippy::too_many_lines)]
    fn try_from(
        raw_uci_message: RawUciMessage<GuiMessagePointer, GuiMessageParameterPointer>,
    ) -> Result<Self, Self::Error> {
        match raw_uci_message.message_pointer {
            // Value-less, parameter-less messages
            GuiMessagePointer::UseUci => Ok(Self::UseUci),
            GuiMessagePointer::IsReady => Ok(Self::IsReady),
            GuiMessagePointer::UciNewGame => Ok(Self::UciNewGame),
            GuiMessagePointer::Stop => Ok(Self::Stop),
            GuiMessagePointer::PonderHit => Ok(Self::PonderHit),
            GuiMessagePointer::Quit => Ok(Self::Quit),
            // Messages with values/parameters
            GuiMessagePointer::Debug => match raw_uci_message
                .value
                .ok_or(Self::Error::MissingValue)?
                .as_bytes()
            {
                b"on" => Ok(Self::Debug(true)),
                b"off" => Ok(Self::Debug(false)),
                _ => Err(Self::Error::ValueParseError),
            },
            GuiMessagePointer::SetOption => {
                let Some(name) = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::SetOption(
                        GuiMessageSetOptionParameterPointer::Name,
                    ))
                    .cloned()
                else {
                    return Err(Self::Error::MissingParameter(
                        GuiMessageParameterPointer::SetOption(
                            GuiMessageSetOptionParameterPointer::Name,
                        ),
                    ));
                };

                let value = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::SetOption(
                        GuiMessageSetOptionParameterPointer::Name,
                    ))
                    .cloned();

                Ok(Self::SetOption(SetOptionMessage { name, value }))
            }
            GuiMessagePointer::Register => {
                if let Some(value) = raw_uci_message.value {
                    if value == "later" {
                        return Ok(Self::Register(RegisterMessageKind::Later));
                    }
                }

                let name = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Register(
                        GuiMessageRegisterParameterPointer::Name,
                    ))
                    .cloned();

                let code = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Register(
                        GuiMessageRegisterParameterPointer::Code,
                    ))
                    .cloned();

                #[allow(clippy::option_if_let_else)]
                if let Some(name) = name {
                    if let Some(code) = code {
                        Ok(Self::Register(RegisterMessageKind::NameAndCode {
                            name,
                            code,
                        }))
                    } else {
                        Ok(Self::Register(RegisterMessageKind::Name(name)))
                    }
                } else if let Some(code) = code {
                    Ok(Self::Register(RegisterMessageKind::Code(code)))
                } else {
                    Err(Self::Error::MissingParameter(
                        GuiMessageParameterPointer::Register(
                            GuiMessageRegisterParameterPointer::Name,
                        ),
                    ))
                }
            }
            GuiMessagePointer::SetPosition => {
                let fen = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::SetPosition(
                        GuiMessageSetPositionParameterPointer::Fen,
                    ))
                    .cloned();

                let moves = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::SetPosition(
                        GuiMessageSetPositionParameterPointer::Moves,
                    ))
                    .and_then(|s| s.parse().ok());

                if let Some(fen) = fen {
                    Ok(Self::SetPosition(SetPositionMessageKind::Fen {
                        fen,
                        moves,
                    }))
                } else {
                    Ok(Self::SetPosition(
                        SetPositionMessageKind::StartingPosition { moves },
                    ))
                }
            }
            GuiMessagePointer::Go => {
                let search_moves = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::SearchMoves,
                    ))
                    .and_then(|s| s.parse().ok());

                let ponder =
                    raw_uci_message
                        .void_parameters
                        .contains(&GuiMessageParameterPointer::Go(
                            GuiMessageGoParameterPointer::Ponder,
                        ));

                let white_time = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::WhiteTime,
                    ))
                    .and_then(|s| s.parse().ok());

                let black_time = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::BlackTime,
                    ))
                    .and_then(|s| s.parse().ok());

                let white_increment = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::WhiteIncrement,
                    ))
                    .and_then(|s| s.parse().ok());

                let black_increment = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::BlackIncrement,
                    ))
                    .and_then(|s| s.parse().ok());

                let moves_to_go = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::MovesToGo,
                    ))
                    .and_then(|s| s.parse().ok());

                let depth = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::Depth,
                    ))
                    .and_then(|s| s.parse().ok());

                let nodes = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::Nodes,
                    ))
                    .and_then(|s| s.parse().ok());

                let mate = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::Mate,
                    ))
                    .and_then(|s| s.parse().ok());

                let move_time = raw_uci_message
                    .parameters
                    .get(&GuiMessageParameterPointer::Go(
                        GuiMessageGoParameterPointer::MoveTime,
                    ))
                    .and_then(|s| s.parse().ok());

                let infinite =
                    raw_uci_message
                        .void_parameters
                        .contains(&GuiMessageParameterPointer::Go(
                            GuiMessageGoParameterPointer::Infinite,
                        ));

                Ok(Self::Go(GoMessage {
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
                }))
            }
        }
    }
}

impl Display for GuiMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseUci => f.write_str("uci")?,
            Self::Debug(value) => write!(f, "debug {}", if *value { "on" } else { "off" })?,
            Self::IsReady => f.write_str("isready")?,
            Self::SetOption(SetOptionMessage {
                value: Some(value),
                name,
            }) => write!(f, "setoption name {name} value {value}")?,
            Self::SetOption(SetOptionMessage { name, .. }) => write!(f, "setoption name {name}")?,
            Self::Register(RegisterMessageKind::Later) => f.write_str("register later")?,
            Self::Register(RegisterMessageKind::Name(name)) => write!(f, "register name {name}")?,
            Self::Register(RegisterMessageKind::Code(code)) => write!(f, "register code {code}")?,
            Self::Register(RegisterMessageKind::NameAndCode { name, code }) => {
                write!(f, "register name {name} code {code}")?;
            }
            Self::UciNewGame => f.write_str("f")?,
            Self::SetPosition(SetPositionMessageKind::StartingPosition { moves: None }) => {
                f.write_str("position startpos")?;
            }
            Self::SetPosition(SetPositionMessageKind::StartingPosition { moves: Some(moves) }) => {
                write!(f, "position startpos moves {}", &moves)?;
            }
            Self::SetPosition(SetPositionMessageKind::Fen {
                fen,
                moves: Some(moves),
            }) => write!(f, "position fen {fen} moves {}", &moves)?,
            Self::SetPosition(SetPositionMessageKind::Fen { fen, .. }) => {
                write!(f, "position fen {fen}")?;
            }
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
            }
            Self::Stop => f.write_str("stop")?,
            Self::PonderHit => f.write_str("ponderhit")?,
            Self::Quit => f.write_str("quit")?,
        }

        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::messages::gui::{GoMessage, GuiMessage};
    use crate::{Message, UciMoveList};
    use pretty_assertions::assert_eq;
    use shakmaty::uci::Uci as UciMove;
    use std::num::NonZeroUsize;

    #[test]
    fn go() {
        let structured_repr = GuiMessage::Go(GoMessage {
            search_moves: Some(UciMoveList(vec![
                UciMove::from_ascii(b"e2e4").unwrap(),
                UciMove::from_ascii(b"d2d4").unwrap(),
            ])),
            ponder: true,
            white_time: Some(5),
            black_time: None,
            white_increment: None,
            black_increment: Some(NonZeroUsize::new(45).unwrap()),
            moves_to_go: None,
            depth: Some(20),
            nodes: None,
            mate: None,
            move_time: None,
            infinite: true,
        });
        let string_repr = "go searchmoves e2e4 d2d4 ponder wtime 5 binc 45 depth 20 infinite\n";

        assert_eq!(structured_repr.to_string(), string_repr);
        assert_eq!(GuiMessage::from_str(string_repr), Ok(structured_repr));
    }
}