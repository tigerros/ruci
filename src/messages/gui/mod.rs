dry_mods::mods! {
    pub mod use go, register, set_option, set_position;
}

// TODO: Tests

use crate::define_message_enum::define_message_enum;
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};
use std::fmt::{Debug, Display, Formatter};

define_message_enum! {
    /// A message sent from the GUI to the engine.
    enum GuiMessage {
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

impl TryFrom<RawUciMessage<Self>> for GuiMessage {
    type Error = MessageTryFromRawUciMessageError<GuiMessageParameterPointer>;

    #[allow(clippy::too_many_lines)]
    fn try_from(raw_uci_message: RawUciMessage<Self>) -> Result<Self, Self::Error> {
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
            GuiMessagePointer::SetOption => Ok(Self::SetOption(SetOptionMessage::try_from(
                raw_uci_message,
            )?)),
            GuiMessagePointer::Register => Ok(Self::Register(RegisterMessageKind::try_from(
                raw_uci_message,
            )?)),
            GuiMessagePointer::SetPosition => Ok(Self::SetPosition(
                SetPositionMessageKind::try_from(raw_uci_message)?,
            )),
            GuiMessagePointer::Go => Ok(Self::Go(GoMessage::try_from(raw_uci_message)?)),
        }
    }
}

impl Display for GuiMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseUci => f.write_str("uci\n"),
            Self::Debug(value) => writeln!(f, "debug {}", if *value { "on" } else { "off" }),
            Self::IsReady => f.write_str("isready\n"),
            Self::SetOption(message) => f.write_str(&message.to_string()),
            Self::Register(kind) => f.write_str(&kind.to_string()),
            Self::UciNewGame => f.write_str("ucinewgame\n"),
            Self::SetPosition(kind) => f.write_str(&kind.to_string()),
            Self::Go(message) => f.write_str(&message.to_string()),
            Self::Stop => f.write_str("stop\n"),
            Self::PonderHit => f.write_str("ponderhit\n"),
            Self::Quit => f.write_str("quit\n"),
        }
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
