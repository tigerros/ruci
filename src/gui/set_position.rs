use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;
use crate::UciMoves;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// <https://backscattering.de/chess/uci/#gui-position>
pub enum SetPosition {
    StartingPosition {
        moves: Option<UciMoves>,
    },
    Fen {
        fen: String,
        moves: Option<UciMoves>,
    },
}

message_from_impl!(gui SetPosition);

impl TryFrom<RawMessage> for SetPosition {
    type Error = MessageParseError;

    fn try_from(raw_message: RawMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != super::pointers::MessagePointer::SetPosition.into() {
            return Err(Self::Error::InvalidMessage);
        };

        let fen = raw_message
            .parameters
            .get(&super::pointers::SetPositionParameterPointer::Fen.into())
            .cloned();

        let moves = raw_message
            .parameters
            .get(&super::pointers::SetPositionParameterPointer::Moves.into())
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
    use shakmaty::uci::UciMove;
    use crate::gui::SetPosition;
    use crate::{Message, UciMoves};

    #[test]
    fn to_from_str() {
        let repr: Message = SetPosition::StartingPosition {
            moves: Some(UciMoves(vec![UciMove::from_ascii(b"d2d4").unwrap(), UciMove::from_ascii(b"d7d5").unwrap()])),
        }.into();

        let str_repr = "position startpos moves d2d4 d7d5\n";
        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }
}