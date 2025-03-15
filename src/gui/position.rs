extern crate alloc;

use core::fmt::{Display, Formatter};
use alloc::string::{String, ToString};
use crate::{parsing, OptionReplaceIf, UciMoves};
use crate::dev_macros::{from_str_parts, message_from_impl};
use crate::gui::pointers::{PositionParameterPointer};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Changes the position to analyze.
/// 
/// <https://backscattering.de/chess/uci/#gui-position>
pub enum Position {
    StartingPosition {
        moves: Option<UciMoves>,
    },
    Fen {
        fen: String,
        moves: Option<UciMoves>,
    },
}

message_from_impl!(gui Position);
from_str_parts!(impl Position for parts -> Self  {
    let mut fen = None;
    let mut moves = None;
    let parameter_fn = |parameter, value: &str| match parameter {
        PositionParameterPointer::Fen => fen = Some(value.to_string()),
        PositionParameterPointer::Moves => moves.replace_if(value.parse().ok()),
    };

    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

    if let Some(fen) = fen {
        Self::Fen {
            fen,
            moves,
        }
    } else {
        Self::StartingPosition { moves }
    }
});

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StartingPosition { moves: None } => f.write_str("position startpos"),
            Self::StartingPosition { moves: Some(moves) } => write!(f, "position startpos moves {}", &moves),
            Self::Fen { fen, moves: Some(moves ) } => write!(f, "position fen {fen} moves {}", &moves),
            Self::Fen { fen, .. } => write!(f, "position fen {fen}")
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use core::str::FromStr;
    use shakmaty::uci::UciMove;
    use alloc::vec;
    use alloc::string::ToString;
    use crate::gui::Position;
    use crate::{Message, UciMoves};
    use pretty_assertions::assert_eq;

    #[test]
    fn to_from_str() {
        let repr: Message = Position::StartingPosition {
            moves: Some(UciMoves(vec![UciMove::from_ascii(b"d2d4").unwrap(), UciMove::from_ascii(b"d7d5").unwrap()])),
        }.into();

        let str_repr = "position startpos moves d2d4 d7d5";
        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }
    
    #[test]
    fn invalid_moves() {
        let m: Message = Position::StartingPosition {
            moves: Some(UciMoves(vec![UciMove::from_ascii(b"d2d4").unwrap()])),
        }.into();

        assert_eq!(m.to_string(), "position startpos moves d2d4");
        assert_eq!(Message::from_str("position startpos moves d2d4 this ain't a move buddy pal"), Ok(m));
    }
}