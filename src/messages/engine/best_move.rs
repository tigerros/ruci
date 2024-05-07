use std::fmt::{Display, Formatter, Write};
use shakmaty::uci::Uci as UciMove;
use crate::messages::{EngineMessage};
use crate::messages::engine::{EngineMessageBestMoveParameterPointer, EngineMessageParameterPointer, EngineMessagePointer};
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-bestmove>
pub struct BestMoveMessage {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}

impl TryFrom<RawUciMessage<EngineMessage>> for BestMoveMessage {
    type Error = MessageTryFromRawUciMessageError<EngineMessageParameterPointer>;

    fn try_from(raw_uci_message: RawUciMessage<EngineMessage>) -> Result<Self, Self::Error> {
        if raw_uci_message.message_pointer != EngineMessagePointer::BestMove {
            return Err(Self::Error::InvalidMessage);
        };
        
        println!("value: {:?}", raw_uci_message.value);

        let Ok(r#move) = raw_uci_message
            .value
            .ok_or(Self::Error::MissingValue)?
            .parse()
            else {
                return Err(Self::Error::ValueParseError);
            };

        let ponder = raw_uci_message
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
    use crate::messages::{EngineMessage, BestMoveMessage};
    use crate::{Message, UciMoveList};
    use shakmaty::uci::Uci as UciMove;
    use pretty_assertions::assert_eq;

    fn repr() -> (EngineMessage, String) {
        (EngineMessage::BestMove(BestMoveMessage {
            r#move: UciMove::from_ascii(b"e2e4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }), "bestmove e2e4 ponder c7c5\n".to_string())
    }

    #[test]
    fn to_string() {
        let repr = repr();

        assert_eq!(repr.0.to_string(), repr.1);
    }

    #[test]
    fn from_string() {
        let repr = repr();

        assert_eq!(EngineMessage::from_str(&repr.1), Ok(repr.0));
    }
}