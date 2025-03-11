use shakmaty::uci::UciMove;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// A simple [`Vec<UciMove>`] wrapper that provides [`FromStr`] and [`Display`] implementations.
pub struct UciMoves(pub Vec<UciMove>);

impl From<Vec<UciMove>> for UciMoves {
    fn from(moves: Vec<UciMove>) -> Self {
        Self(moves)
    }
}

impl FromStr for UciMoves {
    type Err = ();

    /// Splits a string by spaces and keeps parsing the fragments to a [`UciMove`] until it encounters an error,
    /// then returns a unit error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(' ').map_while(|part| part.parse().ok()).collect(),
        ))
    }
}

impl Display for UciMoves {
    /// Just joins the moves with a space.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first_iter = true;

        for r#move in &self.0 {
            // Do not write the space on the first iteration
            if first_iter {
                first_iter = false;
            } else {
                f.write_char(' ')?;
            }

            r#move.fmt(f)?;
        }

        Ok(())
    }
}
