use shakmaty::uci::Uci as UciMove;

#[derive(Debug)]
pub struct BestMoveMessage {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}