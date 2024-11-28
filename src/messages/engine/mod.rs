use super::RawEngineMessage;
dry_mods::mods! {
    // Don't expose the modules of message modules that only have on type in them.
    mod pub use id, best_move, copy_protection, registration;
    pub mod info, option;
}

pub use {info::Info, option::Option};

use crate::{define_message_enum, MessageParseError, MessageTryFromRawMessageError};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

define_message_enum! {
    /// A message sent from the engine to the GUI.
    enum EngineMessage {
        /// <https://backscattering.de/chess/uci/#engine-id>
        %["id"]
        %%[parameters = [(Name, "name"), (Author, "author")]]
        Id(%Id),
        /// <https://backscattering.de/chess/uci/#engine-uciok>
        %["uciok"]
        UciOk,
        /// <https://backscattering.de/chess/uci/#engine-readyok>
        %["readyok"]
        ReadyOk,
        /// <https://backscattering.de/chess/uci/#engine-bestmove>
        %["bestmove"]
        %%[parameters = [(Ponder, "ponder")]]
        BestMove(%BestMove),
        /// <https://backscattering.de/chess/uci/#engine-copyprotection>
        %["copyprotection"]
        CopyProtection(%CopyProtection),
        /// <https://backscattering.de/chess/uci/#engine-registration>
        %["registration"]
        Registration(%Registration),
        /// <https://backscattering.de/chess/uci/#engine-info>
        %["info"]
        %%[parameters = [(Depth, "depth"), (SelectiveSearchDepth, "seldepth"), (Time, "time"), (Nodes, "nodes"), (PrimaryVariation, "pv"), (MultiPrimaryVariation, "multipv"), (Score, "score"), (CurrentMove, "currmove"), (CurrentMoveNumber, "currmovenumber"), (HashFull, "hashfull"), (NodesPerSecond, "nps"), (TableBaseHits, "tbhits"), (ShredderBaseHits, "sbhits"), (CpuLoad, "cpuload"), (String, "string"), (Refutation, "refutation"), (CurrentLine, "currline")]]
        Info(%Box<Info>),
        /// <https://backscattering.de/chess/uci/#engine-option>
        %["option"]
        %%[parameters = [(Name, "name"), (Type, "type"), (Default, "default"), (Min, "min"), (Max, "max"), (Var, "var")]]
        Option(%Option)
    }
}

use pointers::*;

impl FromStr for EngineMessage {
    type Err = MessageParseError<EngineMessageParameterPointer>;

    /// Tries to parse a string into this message.
    ///
    /// # Errors
    ///
    /// See [`MessageParseError`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_message =
            RawEngineMessage::from_str(s).map_err(MessageParseError::RawMessageParseError)?;

        Self::try_from(raw_message).map_err(MessageParseError::MessageTryFromRawMessageError)
    }
}

impl TryFrom<RawEngineMessage> for EngineMessage {
    type Error = MessageTryFromRawMessageError<pointers::EngineMessageParameterPointer>;

    #[allow(clippy::too_many_lines)]
    fn try_from(raw_message: RawEngineMessage) -> Result<Self, Self::Error> {
        match raw_message.message_pointer {
            // Value-less, parameter-less messages
            EngineMessagePointer::UciOk => Ok(Self::UciOk),
            EngineMessagePointer::ReadyOk => Ok(Self::ReadyOk),
            // Messages with values/parameters
            EngineMessagePointer::Id => Ok(Self::Id(Id::try_from(raw_message)?)),
            EngineMessagePointer::BestMove => Ok(Self::BestMove(BestMove::try_from(raw_message)?)),
            EngineMessagePointer::CopyProtection => {
                Ok(Self::CopyProtection(CopyProtection::try_from(raw_message)?))
            }
            EngineMessagePointer::Registration => {
                Ok(Self::Registration(Registration::try_from(raw_message)?))
            }
            EngineMessagePointer::Info => Ok(Self::Info(Box::new(Info::try_from(raw_message)?))),
            EngineMessagePointer::Option => Ok(Self::Option(Option::try_from(raw_message)?)),
        }
    }
}

impl Display for EngineMessage {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(kind) => f.write_str(&kind.to_string()),
            Self::UciOk => f.write_str("uciok\n"),
            Self::ReadyOk => f.write_str("readyok\n"),
            Self::BestMove(message) => f.write_str(&message.to_string()),
            Self::CopyProtection(kind) => f.write_str(&kind.to_string()),
            Self::Registration(kind) => f.write_str(&kind.to_string()),
            Self::Info(info) => f.write_str(&info.to_string()),
            Self::Option(option) => f.write_str(&option.to_string()),
        }
    }
}
