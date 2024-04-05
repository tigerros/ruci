use shakmaty::uci::Uci as UciMove;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct BestMoveMessage {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}