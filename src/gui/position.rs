extern crate alloc;

use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use alloc::string::{String};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use crate::{parsing, uci_moves, MessageParseErrorKind, OptionReplaceIf};
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::errors::MessageParseError;
use crate::gui::pointers::{PositionParameterPointer};
use super::{pointers, traits};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Changes the position to analyze.
///
/// Returns an error when parsing if neither `startpos` nor `fen` parameters are present.
/// If both are present, the first one takes precedence (because Stockfish and Dragon do it like that).
/// 
/// <https://backscattering.de/chess/uci/#gui-position>
pub enum Position<'a> {
    StartPos {
        moves: Cow<'a, [UciMove]>,
    },
    Fen {
        fen: Cow<'a, Fen>,
        moves: Cow<'a, [UciMove]>,
    }
}

impl Position<'_> {
    /// Calls [`Cow::into_owned`] on each [`Cow`] field.
    /// The resulting value has a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Position<'static> {
        match self {
            Self::StartPos { moves } => Position::StartPos { moves: Cow::Owned(moves.into_owned()) },
            Self::Fen { fen, moves } => Position::Fen { fen: Cow::Owned(fen.into_owned()), moves: Cow::Owned(moves.into_owned()) },
        }
    }
}

impl_message!(Position<'_>);
message_from_impl!(gui Position<'a>);
from_str_parts!(impl Position<'_> for parts -> Result {
    let mut startpos = false;
    let mut fen = None::<Fen>;
    let mut moves = Vec::new();
    let parameter_fn = |parameter, _, value: &str, parts| { match parameter {
        PositionParameterPointer::Fen => if !startpos { fen.replace_if(value.parse().ok()); },
        PositionParameterPointer::Moves => {
            let parsed = uci_moves::from_str(value);

            if !parsed.is_empty() {
                moves = parsed;
            }
        },
        PositionParameterPointer::StartPos => if fen.is_none() {
            startpos = true;
        },
    } Some(parts) };

    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

    if let Some(fen) = fen {
        Ok(Self::Fen { fen: Cow::Owned(fen), moves: Cow::Owned(moves) })
    } else if startpos {
        Ok(Self::StartPos { moves: Cow::Owned(moves) })
    } else {
        Err(MessageParseError {
            expected: "startpos",
            kind: MessageParseErrorKind::MissingParameters
        })
    }
});

impl Display for Position<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("position")?;

        match self {
            Self::StartPos { moves } => {
                f.write_str(" startpos")?;

                if !moves.is_empty() {
                    f.write_str(" moves ")?;
                    uci_moves::fmt(moves, f)?;
                }
            }
            Self::Fen { fen, moves } => {
                write!(f, " fen {fen}")?;

                if !moves.is_empty() {
                    f.write_str(" moves ")?;
                    uci_moves::fmt(moves, f)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use alloc::borrow::Cow;
    use shakmaty::uci::UciMove;
    use alloc::string::ToString;
    use crate::gui::Position;
    use shakmaty::fen::Fen;
    use crate::dev_macros::{assert_from_str_message, assert_message_to_from_str, assert_message_to_str};
    use crate::errors::MessageParseError;
    use crate::MessageParseErrorKind;

    #[test]
    fn to_from_str_start_pos() {
        let moves = [UciMove::from_ascii(b"d2d4").unwrap(), UciMove::from_ascii(b"d7d5").unwrap()];
        let repr = Position::StartPos {
            moves: Cow::Borrowed(&moves),
        };

        assert_message_to_from_str!(
            gui 
            repr,
            "position startpos moves d2d4 d7d5"
        );

        let moves = [UciMove::from_ascii(b"d2d4").unwrap(), UciMove::from_ascii(b"d7d5").unwrap()];
        let m = Position::StartPos {
            moves: Cow::Borrowed(&moves),
        };

        assert_from_str_message!(
            gui
            "position    startpos fen rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5 moves \t d2d4 d7d5",
            Ok(m.clone())
        );
        assert_message_to_str!(
            gui
            m,
            "position startpos moves d2d4 d7d5"
        );
    }

    #[test]
    fn to_from_str_fen() {
        let fen = Fen::from_ascii(b"rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5").unwrap();
        let moves = [UciMove::from_ascii(b"b1c3").unwrap()];
        let m = Position::Fen {
            fen: Cow::Borrowed(&fen),
            moves: Cow::Borrowed(&moves),
        };
        
        assert_message_to_from_str!(
            gui 
            m,
            "position fen rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5 moves b1c3"
        );

        let fen = Fen::from_ascii(b"rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5").unwrap();
        let moves = [UciMove::from_ascii(b"b1c3").unwrap()];
        let m = Position::Fen {
            fen: Cow::Borrowed(&fen),
            moves: Cow::Borrowed(&moves),
        };

        assert_from_str_message!(
            gui
            "position fen   rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5 startpos \t moves b1c3",
            Ok(m.clone())
        );
        assert_message_to_str!(
            gui
            m,
            "position fen rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5 moves b1c3"
        );
    }
    
    #[test]
    fn invalid_tail() {
        let moves = [UciMove::from_ascii(b"d2d4").unwrap()];
        
        let m = Position::StartPos {
            moves: Cow::Borrowed(&moves),
        };

        assert_from_str_message!(
            gui
            "position startpos moves d2d4 this ain't a move buddy pal",
            Ok(m.clone())
        );
        assert_message_to_str!(
            gui
            m,
            "position startpos moves d2d4"
        );
    }

    #[test]
    fn parse_error() {
        assert_from_str_message!(
            gui 
            "position moves e2e4",
            Err::<Position, MessageParseError>(MessageParseError {
                expected: "startpos",
                kind: MessageParseErrorKind::MissingParameters
            })
        );
    }
}