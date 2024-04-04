use std::num::NonZeroUsize;
use crate::UciMoveList;

#[derive(Debug)]
pub struct GoMessage {
    /// <https://backscattering.de/chess/uci/#gui-go-searchmoves>
    pub search_moves: Option<UciMoveList>,
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
