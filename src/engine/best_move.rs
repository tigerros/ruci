extern crate alloc;

use core::fmt::{Display, Formatter};
use shakmaty::uci::UciMove;
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, message_from_impl};
use crate::OptionReplaceIf;

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
    let mut r#move = None;
    let mut ponder_encountered = false;
    let mut ponder = None;

    for part in parts {
        if ponder_encountered {
            ponder.replace_if(part.parse().ok());
        } else if part.trim() == "ponder" {
            ponder_encountered = true;
        } else {
            r#move.replace_if(part.parse().ok());
        }
    }
    
    let Some(r#move) = r#move else {
        return Err(MessageParseError::ValueParseError { expected: "UCI move" });
    };

    Ok(Self {
        r#move,
        ponder
    })
});

impl Display for BestMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "bestmove {}", self.r#move)?;

        if let Some(ponder) = &self.ponder {
            write!(f, " ponder {ponder}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use core::str::FromStr;
    use alloc::string::ToString;
    use shakmaty::uci::UciMove;
    use pretty_assertions::{assert_eq};
    use crate::Message;
    use super::BestMove;

    #[test]
    fn to_from_str() {
        let repr: Message = BestMove {
            r#move: UciMove::from_ascii(b"e2e4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }.into();
        let str_repr = "bestmove e2e4 ponder c7c5";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));

        let repr: Message = BestMove {
            r#move: UciMove::from_ascii(b"d2d4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }.into();

        assert_eq!(repr.to_string(), "bestmove d2d4 ponder c7c5");
        assert_eq!(Message::from_str("bestmove oops d2d4 ponder c7c5 ignorthis"), Ok(repr));
    }

    #[test]
    fn to_from_str_bad_value() {
        let repr: Message = BestMove {
            r#move: UciMove::from_ascii(b"e2e4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }.into();
        
        assert_eq!(repr.to_string(), "bestmove e2e4 ponder c7c5");
        assert_eq!(Message::from_str("bestmove junk e2e4 ponder c7c5\n"), Ok(repr));
    }
}