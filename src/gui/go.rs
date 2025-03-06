use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroUsize;
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;
use crate::UciMoves;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl TryFrom<RawMessage> for Go {
    type Error = MessageParseError;

    fn try_from(raw_message: RawMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != super::pointers::MessagePointer::Go.into() {
            return Err(Self::Error::InvalidMessage);
        };

        let search_moves = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::SearchMoves.into())
            .and_then(|s| s.parse().ok());

        let ponder =
            raw_message
                .void_parameters
                .contains(&super::pointers::GoParameterPointer::Ponder.into());

        let white_time = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::WhiteTime.into())
            .and_then(|s| s.parse().ok());

        let black_time = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::BlackTime.into())
            .and_then(|s| s.parse().ok());

        let white_increment = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::WhiteIncrement.into())
            .and_then(|s| s.parse().ok());

        let black_increment = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::BlackIncrement.into())
            .and_then(|s| s.parse().ok());

        let moves_to_go = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::MovesToGo.into())
            .and_then(|s| s.parse().ok());

        let depth = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::Depth.into())
            .and_then(|s| s.parse().ok());

        let nodes = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::Nodes.into())
            .and_then(|s| s.parse().ok());

        let mate = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::Mate.into())
            .and_then(|s| s.parse().ok());

        let move_time = raw_message
            .parameters
            .get(&super::pointers::GoParameterPointer::MoveTime.into())
            .and_then(|s| s.parse().ok());

        let infinite =
            raw_message
                .void_parameters
                .contains(&super::pointers::GoParameterPointer::Infinite.into());

        Ok(Self {
            search_moves,
            ponder,
            white_time,
            black_time,
            white_increment,
            black_increment,
            moves_to_go,
            depth,
            nodes,
            mate,
            move_time,
            infinite,
        })
    }
}

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
