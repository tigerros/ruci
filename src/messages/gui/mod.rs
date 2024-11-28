use super::RawGuiMessage;

dry_mods::mods! {
    mod pub use go, register, set_option, set_position;
}

use crate::define_message_enum::define_message_enum;
use crate::{MessageParseError, MessageTryFromRawMessageError};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

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
        SetOption(%SetOption),
        /// <https://backscattering.de/chess/uci/#gui-register>
        %["register"]
        %%[parameters = [(Name, "name"), (Code, "code")]]
        Register(%Register),
        /// <https://backscattering.de/chess/uci/#gui-ucinewgame>
        %["ucinewgame"]
        UciNewGame,
        /// <https://backscattering.de/chess/uci/#gui-position>
        %["position"]
        %%[parameters = [(Fen, "fen"), (Moves, "moves")]]
        SetPosition(%SetPosition),
        /// <https://backscattering.de/chess/uci/#gui-go>
        %["go"]
        %%[parameters = [(SearchMoves, "searchmoves"), **(Ponder, "ponder"), (WhiteTime, "wtime"), (BlackTime, "btime"), (WhiteIncrement, "winc"), (BlackIncrement, "binc"), (MovesToGo, "movestogo"), (Depth, "depth"), (Nodes, "nodes"), (Mate, "mate"), (MoveTime, "movetime"), **(Infinite, "infinite")]]
        Go(%Go),
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

impl FromStr for GuiMessage {
    type Err = MessageParseError<GuiMessageParameterPointer>;

    /// Tries to parse a string into this message.
    ///
    /// # Errors
    ///
    /// See [`MessageParseError`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_message =
            RawGuiMessage::from_str(s).map_err(MessageParseError::RawMessageParseError)?;

        Self::try_from(raw_message).map_err(MessageParseError::MessageTryFromRawMessageError)
    }
}

impl TryFrom<RawGuiMessage> for GuiMessage {
    type Error = MessageTryFromRawMessageError<GuiMessageParameterPointer>;

    #[allow(clippy::too_many_lines)]
    fn try_from(raw_message: RawGuiMessage) -> Result<Self, Self::Error> {
        match raw_message.message_pointer {
            // Value-less, parameter-less messages
            GuiMessagePointer::UseUci => Ok(Self::UseUci),
            GuiMessagePointer::IsReady => Ok(Self::IsReady),
            GuiMessagePointer::UciNewGame => Ok(Self::UciNewGame),
            GuiMessagePointer::Stop => Ok(Self::Stop),
            GuiMessagePointer::PonderHit => Ok(Self::PonderHit),
            GuiMessagePointer::Quit => Ok(Self::Quit),
            // Messages with values/parameters
            GuiMessagePointer::Debug => match raw_message
                .value
                .ok_or(Self::Error::MissingValue)?
                .as_bytes()
            {
                b"on" => Ok(Self::Debug(true)),
                b"off" => Ok(Self::Debug(false)),
                _ => Err(Self::Error::ValueParseError),
            },
            GuiMessagePointer::SetOption => {
                Ok(Self::SetOption(SetOption::try_from(raw_message)?))
            }
            GuiMessagePointer::Register => {
                Ok(Self::Register(Register::try_from(raw_message)?))
            }
            GuiMessagePointer::SetPosition => Ok(Self::SetPosition(
                SetPosition::try_from(raw_message)?,
            )),
            GuiMessagePointer::Go => Ok(Self::Go(Go::try_from(raw_message)?)),
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
