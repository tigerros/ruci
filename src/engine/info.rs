extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::fmt::{Display, Formatter, Write};
use shakmaty::Color;
use shakmaty::uci::UciMove;
use crate::{parsing, uci_moves, OptionReplaceIf};
use crate::engine::pointers::InfoParameterPointer;
use crate::dev_macros::from_str_parts;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#engine-info-depth>
pub struct Depth {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: usize,
    /// <https://backscattering.de/chess/uci/#engine-info-seldepth>
    pub seldepth: Option<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScoreBound {
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    LowerBound,
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    UpperBound,
}

/// Player advantage/disadvantage from the engine's perspective.
/// 
/// <https://backscattering.de/chess/uci/#engine-info-score>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Score {
    /// <https://backscattering.de/chess/uci/#engine-info-score-centipawns>
    Centipawns(isize),
    /// <https://backscattering.de/chess/uci/#engine-info-score-mate>
    MateIn(isize),
}

impl Score {
    /// Visit [`ScoreStandardized`] to see what this does.
    #[allow(clippy::arithmetic_side_effects)]
    pub const fn standardized(self, turn: Color) -> ScoreStandardized {
        match (turn, self) {
            (Color::White, Self::Centipawns(centipawns)) => ScoreStandardized(Self::Centipawns(centipawns)),
            (Color::Black, Self::Centipawns(centipawns)) => ScoreStandardized(Self::Centipawns(-centipawns)),
            (Color::White, Self::MateIn(mate_in)) => ScoreStandardized(Self::MateIn(mate_in)),
            (Color::Black, Self::MateIn(mate_in)) => ScoreStandardized(Self::MateIn(-mate_in)),
        }
    }
}

/// This struct represents a "standardized" score (read below).
///
/// Engines (should) return a [`Score`] dependent on whose turn it is to move.
/// [`ScoreStandardized`] is a score *independent* of whose turn it is to move.
/// So, `x` means that *white* has an advantage of `x`, `-x` means that *black* has an advantage of `x`.
///
/// Get this by calling [`Score::standardized`].
/// Note that you need to specify whose turn it is to move.
///
/// **⚠️** While the UCI protocol does say engines should score from their point of view,
/// you should verify this is the case with the engine you're using.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreStandardized(Score);

impl ScoreStandardized {
    #[allow(clippy::arithmetic_side_effects)]
    #[doc(hidden)]
    #[deprecated(since = "0.8.1", note = "use Score::standardized instead")]
    pub const fn from_score(score: Score, turn: Color) -> Self {
        score.standardized(turn)
    }

    pub const fn score(self) -> Score {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#engine-info-score>
pub struct ScoreWithBound {
    pub kind: Score,
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    pub bound: Option<ScoreBound>,
}

impl ScoreWithBound {
    fn from_str(s: &str) -> Option<Self> {
        fn isize_at_plus1_position(split: &[&str], position: Option<usize>) -> Option<isize> {
            position.and_then(|position| {
                if position == usize::MAX {
                    None
                } else {
                    // CLIPPY: Potential overflow handled in this if statement
                    #[allow(clippy::arithmetic_side_effects)]
                    split
                        .get(position + 1)
                        .and_then(|s| s.parse().ok())
                }
            })
        }

        let split = s.split(' ').collect::<Vec<_>>();
        let centipawns_position = split.iter().position(|&part| part == "cp");
        let mate_in_position = split.iter().position(|&part| part == "mate");
        let is_lowerbound = split.iter().any(|&part| part == "lowerbound");
        let is_upperbound = split.iter().any(|&part| part == "upperbound");
        let centipawns = isize_at_plus1_position(&split, centipawns_position);
        let mate_in = isize_at_plus1_position(&split, mate_in_position);
        let kind = centipawns.map_or(
            mate_in.map(Score::MateIn),
            |centipawns| Some(Score::Centipawns(centipawns))
        )?;

        Some(Self {
            kind,
            bound: if is_lowerbound && is_upperbound {
                None
            } else if is_lowerbound {
                Some(ScoreBound::LowerBound)
            } else if is_upperbound {
                Some(ScoreBound::UpperBound)
            } else {
                None
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#engine-info-refutation>
pub struct Refutation {
    pub refuted_move: UciMove,
    pub refutation: Vec<UciMove>,
}

impl Refutation {
    fn from_str(s: &str) -> Option<Self> {
        let mut moves = uci_moves::from_str(s);

        if moves.is_empty() {
            return None;
        }

        let first = moves.remove(0);

        Some(Self {
            refuted_move: first,
            refutation: moves,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#engine-info-currline>
pub struct CurrLine {
    pub used_cpu: Option<usize>,
    pub line: Vec<UciMove>,
}

impl CurrLine {
    fn from_str(s: &str) -> Option<Self> {
        // TODO: This will fail if there's more whitespace in between.
        // Fix in future
        let (used_cpu, line) = s.split_once(' ')?;
        let Ok(used_cpu) = used_cpu.parse() else {
            return None;
        };

        Some(Self {
            used_cpu: Some(used_cpu),
            line: uci_moves::from_str(line),
        })
    }
}

/// Information about the engine's calculation.
///
/// Sent (probably multiple times) after [`Go`](crate::gui::Go).
///
/// <https://backscattering.de/chess/uci/#engine-info>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Info {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: Option<Depth>,
    /// <https://backscattering.de/chess/uci/#engine-info-time>
    pub time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-nodes>
    pub nodes: Option<usize>,
    /// Primary variation.
    ///
    /// <https://backscattering.de/chess/uci/#engine-info-pv>
    pub pv: Vec<UciMove>,
    /// Multi primary variation.
    ///
    /// <https://backscattering.de/chess/uci/#engine-info-multipv>
    pub multi_pv: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score>
    pub score: Option<ScoreWithBound>,
    /// Current move.
    ///
    /// <https://backscattering.de/chess/uci/#engine-info-currmove>
    pub curr_move: Option<UciMove>,
    /// Current move number.
    ///
    /// <https://backscattering.de/chess/uci/#engine-info-currmovenumber>
    pub curr_move_number: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-hashfull>
    pub hash_full: Option<usize>,
    /// Nodes per second.
    ///
    /// <https://backscattering.de/chess/uci/#engine-info-nps>
    pub nps: Option<usize>,
    /// Tablebase hits.
    ///
    /// <https://backscattering.de/chess/uci/#engine-info-tbhits>
    pub tb_hits: Option<usize>,
    /// Shredderbase hits.
    ///
    /// <https://backscattering.de/chess/uci/#engine-info-sbhits>
    pub sb_hits: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-cpuload>
    pub cpu_load: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-string>
    pub string: Option<String>,
    /// <https://backscattering.de/chess/uci/#engine-info-refutation>
    pub refutation: Option<Refutation>,
    /// <https://backscattering.de/chess/uci/#engine-info-currline>
    pub curr_line: Option<CurrLine>,
}

impl From<Info> for crate::Message {
    fn from(value: Info) -> Self {
        Self::Engine(crate::engine::Message::Info(Box::new(value)))
    }
}

impl From<Info> for crate::engine::Message {
    fn from(value: Info) -> Self {
        Self::Info(Box::new(value))
    }
}

from_str_parts!(impl Info for parts -> Self {
    let mut this = Self::default();
    // Need to handle depth like this in case the seldepth argument comes before the depth argument.
    let mut depth = None::<usize>;
    let mut seldepth = None::<usize>;
    let parameter_fn = |parameter, value: &str| match parameter {
        InfoParameterPointer::Depth => depth.replace_if(value.parse().ok()),
        InfoParameterPointer::SelDepth => seldepth.replace_if(value.parse().ok()),
        InfoParameterPointer::Time => this.time.replace_if(value.parse().ok()),
        InfoParameterPointer::Nodes => this.nodes.replace_if(value.parse().ok()),
        InfoParameterPointer::PV => {
            let parsed = uci_moves::from_str(value);

            if !parsed.is_empty() {
                this.pv = parsed;
            }
        },
        InfoParameterPointer::MultiPV => this.multi_pv.replace_if(value.parse().ok()),
        InfoParameterPointer::Score => this.score.replace_if(ScoreWithBound::from_str(value)),
        InfoParameterPointer::CurrMove => this.curr_move.replace_if(value.parse().ok()),
        InfoParameterPointer::CurrMoveNumber => this.curr_move_number.replace_if(value.parse().ok()),
        InfoParameterPointer::HashFull => this.hash_full.replace_if(value.parse().ok()),
        InfoParameterPointer::Nps => this.nps.replace_if(value.parse().ok()),
        InfoParameterPointer::TbHits => this.tb_hits.replace_if(value.parse().ok()),
        InfoParameterPointer::SbHits => this.sb_hits.replace_if(value.parse().ok()),
        InfoParameterPointer::CpuLoad => this.cpu_load.replace_if(value.parse().ok()),
        InfoParameterPointer::String => this.string = Some(value.to_string()),
        InfoParameterPointer::Refutation => this.refutation.replace_if(Refutation::from_str(value)),
        InfoParameterPointer::CurrLine => this.curr_line.replace_if(CurrLine::from_str(value)),
    };
    
    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);
    
    if let Some(depth) = depth {
        this.depth = Some(Depth {
            depth,
            seldepth
        });
    }
    
    this
});

impl Display for Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("info")?;

        if let Some(depth) = &self.depth {
            write!(f, " depth {}", depth.depth)?;

            if let Some(selective_search_depth) = depth.seldepth {
                write!(f, " seldepth {selective_search_depth}")?;
            }
        }

        if let Some(time) = self.time {
            write!(f, " time {time}")?;
        }

        if let Some(nodes) = self.nodes {
            write!(f, " nodes {nodes}")?;
        }

        if !self.pv.is_empty() {
            f.write_str(" pv ")?;
            uci_moves::fmt(&self.pv, f)?;
        }

        if let Some(multi_primary_variation) = self.multi_pv {
            write!(f, " multipv {multi_primary_variation}")?;
        }

        if let Some(score) = &self.score {
            f.write_str(" score")?;

            match score.kind {
                Score::Centipawns(centipawns) => write!(f, " cp {centipawns}")?,
                Score::MateIn(mate_in) => write!(f, " mate {mate_in}")?,
            }

            match score.bound {
                Some(ScoreBound::UpperBound) => {
                    f.write_str(" upperbound")?;
                }
                Some(ScoreBound::LowerBound) => {
                    f.write_str(" lowerbound")?;
                }
                None => {}
            }
        }

        if let Some(current_move) = &self.curr_move {
            write!(f, " currmove {current_move}")?;
        }

        if let Some(current_move_number) = self.curr_move_number {
            write!(f, " currmovenumber {current_move_number}")?;
        }

        if let Some(hash_full) = self.hash_full {
            write!(f, " hashfull {hash_full}")?;
        }

        if let Some(nodes_per_second) = self.nps {
            write!(f, " nps {nodes_per_second}")?;
        }

        if let Some(table_base_hits) = self.tb_hits {
            write!(f, " tbhits {table_base_hits}")?;
        }

        if let Some(shredder_base_hits) = self.sb_hits {
            write!(f, " sbhits {shredder_base_hits}")?;
        }

        if let Some(cpu_load) = self.cpu_load {
            write!(f, " cpuload {cpu_load}")?;
        }

        if let Some(string) = &self.string {
            write!(f, " string {string}")?;
        }

        if let Some(refutation) = &self.refutation {
            write!(f, " refutation {} ", refutation.refuted_move)?;
            uci_moves::fmt(&refutation.refutation, f)?;
        }

        if let Some(current_line) = &self.curr_line {
            f.write_str(" currline")?;

            if let Some(used_cpu) = current_line.used_cpu {
                f.write_char(' ')?;
                f.write_str(&used_cpu.to_string())?;
            }
            
            if !current_line.line.is_empty() {
                f.write_char(' ')?;
                uci_moves::fmt(&current_line.line, f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;
    use core::str::FromStr;
    use super::Info;
    use shakmaty::uci::UciMove;
    use pretty_assertions::assert_eq;
    use shakmaty::Color;
    use crate::engine::{CurrLine, Depth, Refutation, Score, ScoreBound, ScoreWithBound};
    use crate::{engine, Message};

    #[test]
    fn score_kind_standardize() {
        assert_eq!(
            Score::Centipawns(-20).standardized(Color::White).score(),
            Score::Centipawns(-20)
        );
        assert_eq!(
            Score::Centipawns(-15).standardized(Color::Black).score(),
            Score::Centipawns(15)
        );
        assert_eq!(
            Score::Centipawns(10).standardized(Color::White).score(),
            Score::Centipawns(10)
        );
        assert_eq!(
            Score::Centipawns(5).standardized(Color::Black).score(),
            Score::Centipawns(-5)
        );
    }

    #[test]
    fn to_from_str() {
        let repr: Message = Info {
            depth: Some(Depth {
                depth: 20,
                seldepth: Some(31)
            }),
            time: Some(12),
            nodes: Some(4),
            pv: vec![UciMove::from_ascii(b"e2e4").unwrap(), UciMove::from_ascii(b"c7c5").unwrap()],
            multi_pv: Some(1),
            score: Some(ScoreWithBound {
                kind: Score::Centipawns(22),
                bound: Some(ScoreBound::LowerBound),
            }),
            curr_move: Some(UciMove::from_ascii(b"e2e4").unwrap()),
            curr_move_number: None,
            hash_full: None,
            nps: None,
            tb_hits: Some(2),
            sb_hits: None,
            cpu_load: None,
            string: Some("blabla".to_string()),
            refutation: Some(Refutation {
                refuted_move: UciMove::from_ascii(b"g2g4").unwrap(),
                refutation: vec![UciMove::from_ascii(b"d7d5").unwrap(), UciMove::from_ascii(b"f1g2").unwrap()],
            }),
            curr_line: Some(CurrLine {
                used_cpu: Some(1),
                line: vec![UciMove::from_ascii(b"e2e4").unwrap(), UciMove::from_ascii(b"c7c5").unwrap()],
            }),
        }.into();
        let str_repr = "info depth 20 seldepth 31 time 12 nodes 4 pv e2e4 c7c5 multipv 1 score cp 22 lowerbound currmove e2e4 tbhits 2 string blabla refutation g2g4 d7d5 f1g2 currline 1 e2e4 c7c5";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }

    #[test]
    fn to_from_str_bad_parameters() {
        let repr: engine::Message = Info {
            depth: Some(Depth {
                depth: 20,
                seldepth: Some(31)
            }),
            time: Some(12),
            nodes: Some(4),
            pv: vec![],
            multi_pv: Some(1),
            score: Some(ScoreWithBound {
                kind: Score::Centipawns(22),
                bound: Some(ScoreBound::LowerBound),
            }),
            curr_move: Some(UciMove::from_ascii(b"e2e4").unwrap()),
            curr_move_number: None,
            hash_full: None,
            nps: None,
            tb_hits: Some(4),
            sb_hits: None,
            cpu_load: None,
            string: Some("blabla".to_string()),
            refutation: Some(Refutation {
                refuted_move: UciMove::from_ascii(b"g2g4").unwrap(),
                refutation: vec![UciMove::from_ascii(b"d7d5").unwrap(), UciMove::from_ascii(b"f1g2").unwrap()],
            }),
            curr_line: Some(CurrLine {
                used_cpu: Some(1),
                line: vec![UciMove::from_ascii(b"e2e4").unwrap(), UciMove::from_ascii(b"c7c5").unwrap()],
            }),
        }.into();

        assert_eq!(repr.to_string(), "info depth 20 seldepth 31 time 12 nodes 4 multipv 1 score cp 22 lowerbound currmove e2e4 tbhits 4 string blabla refutation g2g4 d7d5 f1g2 currline 1 e2e4 c7c5");
        assert_eq!(engine::Message::from_str("info depth BAD depth 20 seldepth 31 time 12 depth also bad nodes 4 multipv 1 score cp 22 lowerbound currmove e2e4 tbhits 2 string blabla refutation g2g4 d7d5 f1g2 currline 1 e2e4 c7c5 tbhits 4"), Ok(repr));
    }
}