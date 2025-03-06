use std::fmt::{Display, Formatter, Write};
use shakmaty::uci::UciMove;
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// <https://backscattering.de/chess/uci/#engine-bestmove>
pub struct BestMove {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}

message_from_impl!(engine BestMove);

impl TryFrom<RawMessage> for BestMove {
    type Error = MessageParseError;

    fn try_from(raw_message: RawMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != super::pointers::MessagePointer::BestMove.into() {
            return Err(Self::Error::InvalidMessage);
        };

        let Ok(r#move) = raw_message
            .value
            .ok_or(Self::Error::MissingValue)?
            .parse()
            else {
                return Err(Self::Error::ValueParseError);
            };

        let ponder = raw_message
            .parameters
            .get(&super::pointers::BestMoveParameterPointer::Ponder.into())
            .and_then(|s| s.parse().ok());

        Ok(Self { r#move, ponder })
    }
}

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