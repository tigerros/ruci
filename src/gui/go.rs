use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroUsize;
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::{parsing, UciMoves};
use crate::from_str_parts::from_str_parts;
use crate::gui::pointers::GoParameterPointer;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#gui-go>
pub struct Go {
    /// <https://backscattering.de/chess/uci/#gui-go-searchmoves>
    pub search_moves: Option<UciMoves>,
    /// <https://backscattering.de/chess/uci/#gui-go-ponder>
    pub ponder: bool,
    /// <https://backscattering.de/chess/uci/#gui-go-wtime>
    pub white_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-btime>
    pub black_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-winc>
    pub white_increment: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-binc>
    pub black_increment: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-movestogo>
    pub moves_to_go: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-depth>
    pub depth: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-nodes>
    pub nodes: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-mate>
    pub mate: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-movetime>
    pub move_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-infinite>
    pub infinite: bool,
}

message_from_impl!(gui Go);
from_str_parts!(impl Go for parts {
    let mut this = Self::default();
    let parameter_fn = |parameter, value: &str| match parameter {
        GoParameterPointer::SearchMoves => this.search_moves = value.parse().ok(),
        GoParameterPointer::Ponder => this.ponder = true,
        GoParameterPointer::WhiteTime => this.white_time = value.parse().ok(),
        GoParameterPointer::BlackTime => this.black_time = value.parse().ok(),
        GoParameterPointer::WhiteIncrement => this.white_increment = value.parse().ok(),
        GoParameterPointer::BlackIncrement => this.black_increment = value.parse().ok(),
        GoParameterPointer::MovesToGo => this.moves_to_go = value.parse().ok(),
        GoParameterPointer::Depth => this.depth = value.parse().ok(),
        GoParameterPointer::Nodes => this.nodes = value.parse().ok(),
        GoParameterPointer::Mate => this.mate = value.parse().ok(),
        GoParameterPointer::MoveTime => this.move_time = value.parse().ok(),
        GoParameterPointer::Infinite => this.infinite = true
    };
    
    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);
    
    Ok(this)
});

impl Display for Go {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("go")?;

        if let Some(search_moves) = &self.search_moves {
            write!(f, " searchmoves {}", &search_moves)?;
        }

        if self.ponder {
            f.write_str(" ponder")?;
        }

        if let Some(white_time) = self.white_time {
            write!(f, " wtime {white_time}")?;
        }

        if let Some(black_time) = self.black_time {
            write!(f, " btime {black_time}")?;
        }

        if let Some(white_increment) = self.white_increment {
            write!(f, " winc {white_increment}")?;
        }

        if let Some(black_increment) = self.black_increment {
            write!(f, " binc {black_increment}")?;
        }

        if let Some(moves_to_go) = self.moves_to_go {
            write!(f, " moves_to_go {moves_to_go}")?;
        }

        if let Some(depth) = self.depth {
            write!(f, " depth {depth}")?;
        }

        if let Some(nodes) = self.nodes {
            write!(f, " nodes {nodes}")?;
        }

        if let Some(mate) = self.mate {
            write!(f, " mate {mate}")?;
        }

        if let Some(move_time) = self.move_time {
            write!(f, " movetime {move_time}")?;
        }

        if self.infinite {
            f.write_str(" infinite")?;
        }
        
        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use pretty_assertions::assert_eq;
    use shakmaty::uci::UciMove;
    use std::num::NonZeroUsize;
    use std::str::FromStr;
    use crate::gui::Go;
    use crate::{Message, UciMoves};

    #[test]
    fn to_from_str() {
        let repr: Message = Go {
            search_moves: Some(UciMoves(vec![
                UciMove::from_ascii(b"e2e4").unwrap(),
                UciMove::from_ascii(b"d2d4").unwrap(),
            ])),
            ponder: true,
            white_time: Some(5),
            black_time: None,
            white_increment: None,
            black_increment: Some(NonZeroUsize::new(45).unwrap()),
            moves_to_go: None,
            depth: Some(20),
            nodes: None,
            mate: None,
            move_time: None,
            infinite: true,
        }.into();
        let str_repr = "go searchmoves e2e4 d2d4 ponder wtime 5 binc 45 depth 20 infinite\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }
}
