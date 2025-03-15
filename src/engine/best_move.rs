extern crate alloc;

use alloc::string::String;
use core::fmt::{Display, Formatter, Write};
use core::str::FromStr;
use shakmaty::uci::UciMove;
use crate::engine::pointers::{BestMoveParameterPointer};
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, message_from_impl};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The engine's best move, with an optional pondering move.
/// 
/// Sent after [`Go`](crate::gui::Go) is received and calculation is finished.
/// 
/// <https://backscattering.de/chess/uci/#engine-bestmove>
pub struct BestMove {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}

message_from_impl!(engine BestMove);
from_str_parts!(impl BestMove for parts -> Result<Self, MessageParseError>  {
    let mut move_value = String::with_capacity(50);
    let mut ponder = None::<String>;

    for part in parts {
        if let Some(ponder) = &mut ponder {
            ponder.push_str(part.trim());
        } else if BestMoveParameterPointer::from_str(part.trim()).is_ok() {
            ponder = Some(String::with_capacity(50));
        } else {
            move_value.push_str(part.trim());
        }
    }

    Ok(Self {
        r#move: move_value.parse().map_err(|_| MessageParseError::ValueParseError { expected: "UCI move" })?,
        ponder: ponder.and_then(|p| p.parse().ok())
    })
});

impl Display for BestMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "bestmove {}", self.r#move)?;

        if let Some(ponder) = &self.ponder {
            write!(f, " ponder {ponder}")?;
        }
        
        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use core::str::FromStr;
    use alloc::string::ToString;
    use shakmaty::uci::UciMove;
    use pretty_assertions::{assert_eq, assert_matches};
    use crate::errors::MessageParseError;
    use crate::Message;
    use super::BestMove;

    #[test]
    fn to_from_str() {
        let repr: Message = BestMove {
            r#move: UciMove::from_ascii(b"e2e4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }.into();
        let str_repr = "bestmove e2e4 ponder c7c5\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));

        let repr: Message = BestMove {
            r#move: UciMove::from_ascii(b"d2d4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }.into();
        let str_repr = "bestmove d2d4 ponder c7c5 ignore this\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }

    #[test]
    fn parse_error() {
        assert_matches!(Message::from_str("bestmove notvalid e2e4 ponder c7c5\n"), Err(MessageParseError::ValueParseError { .. }));
    }
}