use crate::UciMoveList;

#[derive(Debug)]
pub enum SetPositionMessageKind {
    StartingPosition {
        moves: Option<UciMoveList>,
    },
    Fen {
        fen: String,
        moves: Option<UciMoveList>,
    },
}