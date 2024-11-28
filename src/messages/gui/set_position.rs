use std::fmt::{Display, Formatter, Write};
use crate::{MessageTryFromRawMessageError, UciMoveList};
use crate::messages::{GuiMessageParameterPointer, GuiMessagePointer, GuiMessageSetPositionParameterPointer};
use crate::messages::RawGuiMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#gui-position>
pub enum SetPosition {
    StartingPosition {
        moves: Option<UciMoveList>,
    },
    Fen {
        fen: String,
        moves: Option<UciMoveList>,
    },
}

impl TryFrom<RawGuiMessage> for SetPosition {
    type Error = MessageTryFromRawMessageError<GuiMessageParameterPointer>;

    fn try_from(raw_message: RawGuiMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != GuiMessagePointer::SetPosition {
            return Err(Self::Error::InvalidMessage);
        };

        let fen = raw_message
            .parameters
            .get(&GuiMessageParameterPointer::SetPosition(
                GuiMessageSetPositionParameterPointer::Fen,
            ))
            .cloned();

        let moves = raw_message
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

impl Display for SetPosition {
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::str::FromStr;
    use crate::messages::{GuiMessage, SetPosition};
    use crate::{UciMoveList};
    use shakmaty::uci::UciMove;

    #[test]
    fn to_from_str() {
        let repr = GuiMessage::SetPosition(SetPosition::StartingPosition {
            moves: Some(UciMoveList(vec![UciMove::from_ascii(b"d2d4").unwrap(), UciMove::from_ascii(b"d7d5").unwrap()])),
        });
        let str_repr = "position startpos moves d2d4 d7d5\n";
        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(GuiMessage::from_str(str_repr), Ok(repr));
    }
}