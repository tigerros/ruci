use std::fmt::{Display, Formatter, Write};
use shakmaty::uci::Uci as UciMove;
use crate::messages::engine::{EngineMessageBestMoveParameterPointer, EngineMessageParameterPointer, EngineMessagePointer};
use crate::{MessageTryFromRawMessageError};
use crate::messages::engine::raw_engine_message::RawEngineMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-bestmove>
pub struct BestMoveMessage {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}

impl TryFrom<RawEngineMessage> for BestMoveMessage {
    type Error = MessageTryFromRawMessageError<EngineMessageParameterPointer>;

    fn try_from(raw_message: RawEngineMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != EngineMessagePointer::BestMove {
            return Err(Self::Error::InvalidMessage);
        };
        
        println!("value: {:?}", raw_message.value);

        let Ok(r#move) = raw_message
            .value
            .ok_or(Self::Error::MissingValue)?
            .parse()
            else {
                return Err(Self::Error::ValueParseError);
            };

        let ponder = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::BestMove(
                EngineMessageBestMoveParameterPointer::Ponder,
            ))
            .and_then(|s| s.parse().ok());

        Ok(Self { r#move, ponder })
    }
}

impl Display for BestMoveMessage {
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
    use crate::messages::{EngineMessage, BestMoveMessage};
    
    use shakmaty::uci::Uci as UciMove;
    use pretty_assertions::assert_eq;

    #[test]
    fn to_from_str() {
        let repr = EngineMessage::BestMove(BestMoveMessage {
            r#move: UciMove::from_ascii(b"e2e4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        });
        let str_repr = "bestmove e2e4 ponder c7c5\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(EngineMessage::from_str(str_repr), Ok(repr));
    }
}