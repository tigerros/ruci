extern crate alloc;

use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use alloc::string::{String};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use crate::{parsing, uci_moves, OptionReplaceIf};
use crate::dev_macros::{from_str_parts, message_from_impl};
use crate::gui::pointers::{PositionParameterPointer};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Changes the position to analyze.
/// 
/// <https://backscattering.de/chess/uci/#gui-position>
pub struct Position {
    pub startpos: bool,
    pub fen: Option<Fen>,
    pub moves: Vec<UciMove>,
}

message_from_impl!(gui Position);
from_str_parts!(impl Position for parts -> Self  {
    let mut startpos = false;
    let mut fen = None;
    let mut moves = Vec::new();
    let parameter_fn = |parameter, value: &str| match parameter {
        PositionParameterPointer::Fen => fen.replace_if(value.parse().ok()),
        PositionParameterPointer::Moves => {
            let parsed = uci_moves::from_str(value);

            if !parsed.is_empty() {
                moves = parsed;
            }
        },
        PositionParameterPointer::StartPos => startpos = true,
    };

    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

    Self {
        startpos,
        fen,
        moves
    }
});

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("position")?;

        if self.startpos {
            f.write_str(" startpos")?;
        }

        if let Some(fen) = &self.fen {
            write!(f, " fen {fen}")?;
        }

        if !self.moves.is_empty() {
            f.write_str(" moves ")?;
            uci_moves::fmt(&self.moves, f)?;
        }

        Ok(())
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
    use crate::{gui, Message};
    use pretty_assertions::assert_eq;

    #[test]
    fn to_from_str() {
        let repr: Message = Position {
            startpos: true,
            fen: None,
            moves: vec![UciMove::from_ascii(b"d2d4").unwrap(), UciMove::from_ascii(b"d7d5").unwrap()],
        }.into();

        let str_repr = "position startpos moves d2d4 d7d5";
        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }
    
    #[test]
    fn invalid_tail() {
        let m: gui::Message = Position {
            startpos: false,
            fen: None,
            moves: vec![UciMove::from_ascii(b"d2d4").unwrap()],
        }.into();

        assert_eq!(m.to_string(), "position moves d2d4");
        assert_eq!(gui::Message::from_str("position moves d2d4 this ain't a move buddy pal"), Ok(m));
    }
}