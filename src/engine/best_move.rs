use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use shakmaty::uci::UciMove;
use crate::engine::pointers::{BestMoveParameterPointer};
use crate::errors::MessageParseError;
use crate::from_str_parts::from_str_parts;
use crate::message_from_impl::message_from_impl;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#engine-bestmove>
pub struct BestMove {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}

message_from_impl!(engine BestMove);
from_str_parts!(impl BestMove for parts {
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
        r#move: move_value.parse().map_err(|_| MessageParseError::ValueParseError)?,
        ponder: ponder.and_then(|p| p.parse().ok())
    })
});

impl Display for BestMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    use std::str::FromStr;
    
    use shakmaty::uci::UciMove;
    use pretty_assertions::assert_eq;
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
    }
}