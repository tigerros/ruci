use std::fmt::{Display, Formatter, Write};
use crate::messages::{GoMessage, GuiMessage};
use crate::{MessageTryFromRawUciMessageError, RawUciMessage, UciMoveList};
use crate::messages::engine::EngineMessageParameterPointer;
use crate::messages::gui::{GuiMessageParameterPointer, GuiMessagePointer, GuiMessageSetPositionParameterPointer};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#gui-position>
pub enum SetPositionMessageKind {
    StartingPosition {
        moves: Option<UciMoveList>,
    },
    Fen {
        fen: String,
        moves: Option<UciMoveList>,
    },
}

impl TryFrom<RawUciMessage<GuiMessage>> for SetPositionMessageKind {
    type Error = MessageTryFromRawUciMessageError<GuiMessageParameterPointer>;

    fn try_from(raw_uci_message: RawUciMessage<GuiMessage>) -> Result<Self, Self::Error> {
        if raw_uci_message.message_pointer != GuiMessagePointer::SetPosition {
            return Err(Self::Error::InvalidMessage);
        };

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
            Ok(Self::Fen {
                fen,
                moves,
            })
        } else {
            Ok(Self::StartingPosition { moves })
        }
    }
}

impl Display for SetPositionMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartingPosition { moves: None } => f.write_str("position startpos")?,
            Self::StartingPosition { moves: Some(moves) } => write!(f, "position startpos moves {}", &moves)?,
            Self::Fen {
                fen,
                moves: None,
            } => write!(f, "position fen {fen}")?,
            Self::Fen { fen, moves: Some(moves ) } => write!(f, "position fen {fen} moves {}", &moves)?,
        }
        
        f.write_char('\n')
    }
}