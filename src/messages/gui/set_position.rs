use crate::UciMoveList;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetPositionMessageKind {
    StartingPosition {
        moves: Option<UciMoveList>,
    },
    Fen {
        fen: String,
        moves: Option<UciMoveList>,
    },
}