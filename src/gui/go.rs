extern crate alloc;

use alloc::borrow::Cow;
use alloc::vec::Vec;
use alloc::string::String;
use core::fmt::{Display, Formatter};
use core::num::NonZeroUsize;
use shakmaty::uci::UciMove;
use crate::{parsing, uci_moves, OptionReplaceIf};
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::gui::pointers::GoParameterPointer;
use super::{pointers, traits};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Tells the engine to start calculating.
///
/// <https://backscattering.de/chess/uci/#gui-go>
pub struct Go<'a> {
    /// <https://backscattering.de/chess/uci/#gui-go-searchmoves>
    pub search_moves: Cow<'a, [UciMove]>,
    /// <https://backscattering.de/chess/uci/#gui-go-ponder>
    pub ponder: bool,
    /// White's time.
    ///
    /// <https://backscattering.de/chess/uci/#gui-go-wtime>
    pub w_time: Option<usize>,
    /// Black's time.
    ///
    /// <https://backscattering.de/chess/uci/#gui-go-btime>
    pub b_time: Option<usize>,
    /// White's increment.
    ///
    /// <https://backscattering.de/chess/uci/#gui-go-winc>
    pub w_inc: Option<NonZeroUsize>,
    /// Black's increment.
    ///
    /// <https://backscattering.de/chess/uci/#gui-go-binc>
    pub b_inc: Option<NonZeroUsize>,
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

impl_message!(Go<'_>);
message_from_impl!(gui Go<'a>);
from_str_parts!(impl Go<'_> for parts -> Self {
    let mut this = Self::default();
    let mut search_moves = Vec::new();
    let parameter_fn = |parameter, _, value: &str, parts| {
        match parameter {
            GoParameterPointer::SearchMoves => {
                let parsed = uci_moves::from_str(value);
    
                if !parsed.is_empty() {
                    search_moves = parsed;
                }
            },
            GoParameterPointer::Ponder => this.ponder = true,
            GoParameterPointer::WTime => this.w_time.replace_if(value.parse().ok()),
            GoParameterPointer::BTime => this.b_time.replace_if(value.parse().ok()),
            GoParameterPointer::WInc => this.w_inc.replace_if(value.parse().ok()),
            GoParameterPointer::BInc => this.b_inc.replace_if(value.parse().ok()),
            GoParameterPointer::MovesToGo => this.moves_to_go.replace_if(value.parse().ok()),
            GoParameterPointer::Depth => this.depth.replace_if(value.parse().ok()),
            GoParameterPointer::Nodes => this.nodes.replace_if(value.parse().ok()),
            GoParameterPointer::Mate => this.mate.replace_if(value.parse().ok()),
            GoParameterPointer::MoveTime => this.move_time.replace_if(value.parse().ok()),
            GoParameterPointer::Infinite => this.infinite = true
        }
        
        Some(parts)
    };
    
    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

    this.search_moves = Cow::from(search_moves);
    
    this
});

impl Display for Go<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("go")?;

        if !self.search_moves.is_empty() {
            f.write_str(" searchmoves ")?;
            uci_moves::fmt(&self.search_moves, f)?;
        }

        if self.ponder {
            f.write_str(" ponder")?;
        }

        if let Some(white_time) = self.w_time {
            write!(f, " wtime {white_time}")?;
        }

        if let Some(black_time) = self.b_time {
            write!(f, " btime {black_time}")?;
        }

        if let Some(white_increment) = self.w_inc {
            write!(f, " winc {white_increment}")?;
        }

        if let Some(black_increment) = self.b_inc {
            write!(f, " binc {black_increment}")?;
        }

        if let Some(moves_to_go) = self.moves_to_go {
            write!(f, " movestogo {moves_to_go}")?;
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
        
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use alloc::string::ToString;
    use alloc::borrow::Cow;
    use shakmaty::uci::UciMove;
    use core::num::NonZeroUsize;
    use crate::gui::Go;
    use crate::dev_macros::{assert_from_str_message, assert_message_to_from_str, assert_message_to_str};

    #[test]
    fn to_from_str() {
        let search_moves = [
            UciMove::from_ascii(b"e2e4").unwrap(),
            UciMove::from_ascii(b"d2d4").unwrap(),
        ];

        let m = Go {
            search_moves: Cow::Borrowed(&search_moves),
            ponder: true,
            w_time: Some(5),
            b_time: None,
            w_inc: None,
            b_inc: Some(NonZeroUsize::new(45).unwrap()),
            moves_to_go: None,
            depth: Some(20),
            nodes: None,
            mate: None,
            move_time: None,
            infinite: true,
        };

        assert_message_to_from_str!(
            gui
            m,
            "go searchmoves e2e4 d2d4 ponder wtime 5 binc 45 depth 20 infinite"
        );
    }

    #[test]
    fn to_from_str_bad_parameters() {
        let m = Go {
            search_moves: Cow::from(&[]),
            ponder: true,
            w_time: None,
            b_time: None,
            w_inc: None,
            b_inc: None,
            moves_to_go: None,
            depth: Some(20),
            nodes: Some(2),
            mate: Some(0),
            move_time: None,
            infinite: false,
        };

        assert_from_str_message!(
            gui
            "go mate 7 ponder depth 20 depth bad nodes nope nope nodes 2 mate 0",
            Ok(m.clone())
        );
        assert_message_to_str!(
            gui
            m,
            "go ponder depth 20 nodes 2 mate 0"
        );
    }

    #[test]
    fn to_from_str_empty() {
        let m = Go::default();

        assert_from_str_message!(
            gui
            "     go    this is  empty)",
            Ok(m.clone())
        );
        assert_message_to_str!(
            gui
            m,
            "go"
        );
    }

    #[test]
    fn time_controls() {
        let m = Go {
            search_moves: Cow::from(&[]),
            ponder: false,
            w_time: Some(1000),
            b_time: Some(1500),
            w_inc: Some(NonZeroUsize::new(10).unwrap()),
            b_inc: Some(NonZeroUsize::new(10).unwrap()),
            moves_to_go: Some(NonZeroUsize::new(40).unwrap()),
            depth: None,
            nodes: None,
            mate: None,
            move_time: None,
            infinite: false,
        };

        assert_message_to_from_str!(
            gui
            m,
            "go wtime 1000 btime 1500 winc 10 binc 10 movestogo 40"
        );
    }

    #[test]
    fn search_limits() {
        let m = Go {
            search_moves: Cow::from(&[]),
            ponder: false,
            w_time: None,
            b_time: None,
            w_inc: None,
            b_inc: None,
            moves_to_go: None,
            depth: Some(5),
            nodes: Some(1_000_000),
            mate: Some(3),
            move_time: Some(5000),
            infinite: false,
        };

        assert_message_to_from_str!(
            gui
            m,
            "go depth 5 nodes 1000000 mate 3 movetime 5000"
        );
    }

    #[test]
    fn search_moves_only() {
        let search_moves = [
            UciMove::from_ascii(b"e2e4").unwrap(),
            UciMove::from_ascii(b"e7e5").unwrap(),
            UciMove::from_ascii(b"f1c4").unwrap(),
        ];

        let m = Go {
            search_moves: Cow::Borrowed(&search_moves),
            ponder: false,
            w_time: None,
            b_time: None,
            w_inc: None,
            b_inc: None,
            moves_to_go: None,
            depth: None,
            nodes: None,
            mate: None,
            move_time: None,
            infinite: false,
        };

        assert_message_to_from_str!(
            gui
            m,
            "go searchmoves e2e4 e7e5 f1c4"
        );
    }

    #[test]
    fn invalid_parameters() {
        // Test that invalid numeric parameters are ignored
        assert_from_str_message!(
            gui
            "go depth -1 nodes -5 mate abc movetime def",
            Ok(Go::default())
        );

        // Test that unknown parameters are ignored
        assert_from_str_message!(
            gui
            "go unknown_param 123 another_param xyz depth 10",
            Ok(Go {
                depth: Some(10),
                ..Go::default()
            })
        );
    }

    #[test]
    fn movetime_only() {
        let m = Go {
            search_moves: Cow::from(&[]),
            ponder: false,
            w_time: None,
            b_time: None,
            w_inc: None,
            b_inc: None,
            moves_to_go: None,
            depth: None,
            nodes: None,
            mate: None,
            move_time: Some(15000),
            infinite: false,
        };

        assert_message_to_from_str!(
            gui
            m,
            "go movetime 15000"
        );
    }
}