use std::fmt::{Display, Formatter, Write};
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

impl Display for UciMoveList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first_iter = true;
        
        for r#move in &self.0 {
            // Don't write the space on the first iteration
            if first_iter {
                first_iter = false;
            } else {
                f.write_char(' ')?;
            }
            
            f.write_str(&r#move.to_string())?;
        }
        
        Ok(())
    }
}
