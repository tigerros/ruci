extern crate alloc;

use core::fmt::{Display, Formatter, Write};
use alloc::string::{String, ToString};
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::{parsing, UciMoves};
use crate::from_str_parts::from_str_parts;
use crate::gui::pointers::{SetPositionParameterPointer};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
from_str_parts!(impl SetPosition for parts -> Self  {
    let mut fen = None;
    let mut moves = None;
    let parameter_fn = |parameter, value: &str| match parameter {
        SetPositionParameterPointer::Fen => fen = Some(value.to_string()),
        SetPositionParameterPointer::Moves => moves = value.parse().ok(),
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

impl Display for SetPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StartingPosition { moves: None } => f.write_str("position startpos")?,
            Self::StartingPosition { moves: Some(moves) } => write!(f, "position startpos moves {}", &moves)?,
            Self::Fen { fen, moves: Some(moves ) } => write!(f, "position fen {fen} moves {}", &moves)?,
            Self::Fen { fen, .. } => write!(f, "position fen {fen}")?,
        }
        
        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use core::str::FromStr;
    use shakmaty::uci::UciMove;
    use alloc::vec;
    use alloc::string::ToString;
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
    
    #[test]
    fn invalid_moves() {
        let m: Message = SetPosition::StartingPosition {
            moves: Some(UciMoves(vec![UciMove::from_ascii(b"d2d4").unwrap()])),
        }.into();

        assert_eq!(m.to_string(), "position startpos moves d2d4\n");
        assert_eq!(Message::from_str("position startpos moves d2d4 this ain't a move buddy pal\n"), Ok(m));
    }
}