extern crate alloc;

use core::fmt::{Display, Formatter};
use shakmaty::uci::UciMove;
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::OptionReplaceIf;
use super::{pointers, traits};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BestMove {
    /// This variant exists because engines can send "valid" [`BestMove`] messages, which will fail to parse.
    ///
    /// For example, this happens with Stockfish when trying to analyze a game over position, it will send back `bestmove (none)`.
    /// However, Komodo Dragon sends back a null move; `bestmove 0000`.
    ///
    /// This variant just means that the `bestmove` string
    /// was encountered, but the rest of the message was not understood.
    ///
    /// The [`Display`] impl of this variant is just `"bestmove"`.
    ///
    /// This case is not covered by the protocol description, which is why this solution
    /// is improvised and isn't great.
    Other,
    Normal(NormalBestMove),
}

impl BestMove {
    /// Returns the inner [`NormalBestMove`], if [`Self`] matches [`Self::Normal`].
    pub const fn normal(self) -> Option<NormalBestMove> {
        match self {
            Self::Other => None,
            Self::Normal(n) => Some(n),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The engine's best move, with an optional pondering move.
///
/// Sent after [`Go`](crate::gui::Go) is received and calculation is finished.
///
/// <https://backscattering.de/chess/uci/#engine-bestmove>
pub struct NormalBestMove {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}

impl From<NormalBestMove> for BestMove {
    fn from(value: NormalBestMove) -> Self {
        Self::Normal(value)
    }
}

impl From<NormalBestMove> for crate::Message<'_> {
    fn from(value: NormalBestMove) -> Self {
        Self::Engine(super::Message::BestMove(value.into()))
    }
}

impl From<NormalBestMove> for super::Message<'_> {
    fn from(value: NormalBestMove) -> Self {
        Self::BestMove(value.into())
    }
}

impl_message!(BestMove);
message_from_impl!(engine BestMove);
from_str_parts!(impl BestMove for parts -> Self  {
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

    r#move.map_or(Self::Other, |r#move| Self::Normal(NormalBestMove { r#move, ponder }))
});

impl Display for BestMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("bestmove")?;

        match &self {
            Self::Other => Ok(()),
            Self::Normal(best_move) => {
                write!(f, " {}", best_move.r#move)?;

                best_move.ponder.as_ref().map_or(Ok(()), |ponder| write!(f, " ponder {ponder}"))
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use alloc::string::ToString;
    use shakmaty::uci::UciMove;
    use crate::engine::{BestMove, NormalBestMove};
    use crate::dev_macros::{assert_from_str_message, assert_message_to_from_str, assert_message_to_str};

    #[test]
    fn to_from_str() {
        let m: BestMove = NormalBestMove {
            r#move: UciMove::from_ascii(b"e2e4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }.into();
        
        assert_message_to_from_str!(engine m, "bestmove e2e4 ponder c7c5");
        
        let m: BestMove = NormalBestMove {
            r#move: UciMove::from_ascii(b"d2d4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }.into();

        assert_from_str_message!(engine "bestmove oops d2d4 ponder c7c5 ignorethis", Ok(m));
        assert_message_to_str!(engine m, "bestmove d2d4 ponder c7c5");
    }

    #[test]
    fn to_from_str_bad_value() {
        let m: BestMove = NormalBestMove {
            r#move: UciMove::from_ascii(b"e2e4").unwrap(),
            ponder: Some(UciMove::from_ascii(b"c7c5").unwrap()),
        }.into();

        assert_from_str_message!(engine "bestmove junk e2e4 ponder c7c5\n", Ok(m));
        assert_message_to_str!(engine m, "bestmove e2e4 ponder c7c5");
    }

    #[test]
    fn to_from_str_other() {
        let m: BestMove = BestMove::Other;

        assert_from_str_message!(engine "bestmove (none)\n", Ok(m));
        assert_message_to_str!(engine m, "bestmove");
    }
}