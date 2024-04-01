use shakmaty::uci::Uci as UciMove;
use std::str::FromStr;

#[derive(Debug)]
pub struct UciMoveList(pub Vec<UciMove>);

impl FromStr for UciMoveList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(' ').map_while(|part| part.parse().ok()).collect(),
        ))
    }
}
