use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroUsize;
use shakmaty::Color;
use crate::{MessageTryFromRawMessageError, UciMoveList};
use shakmaty::uci::UciMove;
use crate::messages::RawEngineMessage;
use crate::messages::pointers::engine::*;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-info-depth>
pub struct Depth {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: usize,
    /// <https://backscattering.de/chess/uci/#engine-info-seldepth>
    pub selective_search_depth: Option<usize>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScoreBound {
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    Lowerbound,
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    Upperbound,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScoreKind {
    /// <https://backscattering.de/chess/uci/#engine-info-score-centipawns>
    Centipawns(isize),
    /// <https://backscattering.de/chess/uci/#engine-info-score-mate>
    MateIn(isize),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Centipawns {
    Equal,
    White(NonZeroUsize),
    Black(NonZeroUsize)
}

/// Which color has mate in how many moves.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MateIn {
    White(usize),
    Black(usize)
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScoreKindStandardized {
    Centipawns(Centipawns),
    MateIn(MateIn),
}

impl ScoreKind {
    /// The centipawn and mate scores are dependent on whose turn it is to move.
    ///
    /// If it is white's turn, and the score is `-x`, it means that *black* has an advantage of `x`.
    /// However, if it is black's turn, and the score is `-x`, it means that *white* has an advantage of `x`.
    ///
    /// This function returns a "standardized" score.
    /// A positive score means that white has the advantage, and a negative score means that
    /// black has the advantage.
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)]
    pub const fn standardized(self, turn: Color) -> ScoreKindStandardized {
        match (turn, self) {
            (Color::White, Self::Centipawns(centipawns)) => match centipawns {
                0 => ScoreKindStandardized::Centipawns(Centipawns::Equal),
                1.. => ScoreKindStandardized::Centipawns(Centipawns::White(unsafe { NonZeroUsize::new_unchecked(centipawns as usize) })),
                ..=-1 => ScoreKindStandardized::Centipawns(Centipawns::Black(unsafe { NonZeroUsize::new_unchecked(centipawns.abs() as usize) }))
            },
            (Color::Black, Self::Centipawns(centipawns)) => match centipawns {
                0 => ScoreKindStandardized::Centipawns(Centipawns::Equal),
                1.. => ScoreKindStandardized::Centipawns(Centipawns::Black(unsafe { NonZeroUsize::new_unchecked(centipawns as usize) })),
                ..=-1 => ScoreKindStandardized::Centipawns(Centipawns::White(unsafe { NonZeroUsize::new_unchecked(centipawns.abs() as usize) }))
            },
            (Color::White, Self::MateIn(mate_in)) => if mate_in >= 0 {
                ScoreKindStandardized::MateIn(MateIn::White(mate_in as usize))
            } else {
                ScoreKindStandardized::MateIn(MateIn::Black(mate_in as usize))
            },
            (Color::Black, Self::MateIn(mate_in)) => if mate_in >= 0 {
                ScoreKindStandardized::MateIn(MateIn::Black(mate_in as usize))
            } else {
                ScoreKindStandardized::MateIn(MateIn::White(mate_in as usize))
            },
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-info-score>
pub struct Score {
    pub kind: ScoreKind,
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    pub bound: Option<ScoreBound>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-info-refutation>
pub struct Refutation {
    pub refuted_move: UciMove,
    pub refutation: UciMoveList,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-info-currline>
pub struct CurrentLine {
    pub used_cpu: Option<usize>,
    pub line: UciMoveList,
}

/// <https://backscattering.de/chess/uci/#engine-info>
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: Option<Depth>,
    /// <https://backscattering.de/chess/uci/#engine-info-time>
    pub time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-nodes>
    pub nodes: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-pv>
    pub primary_variation: Option<UciMoveList>,
    /// <https://backscattering.de/chess/uci/#engine-info-multipv>
    pub multi_primary_variation: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score>
    pub score: Option<Score>,
    /// <https://backscattering.de/chess/uci/#engine-info-currmove>
    pub current_move: Option<UciMove>,
    /// <https://backscattering.de/chess/uci/#engine-info-currmovenumber>
    pub current_move_number: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-hashfull>
    pub hash_full: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-nps>
    pub nodes_per_second: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-tbhits>
    pub table_base_hits: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-sbhits>
    pub shredder_base_hits: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-cpuload>
    pub cpu_load: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-string>
    pub string: Option<String>,
    /// <https://backscattering.de/chess/uci/#engine-info-refutation>
    pub refutation: Option<Refutation>,
    /// <https://backscattering.de/chess/uci/#engine-info-currline>
    pub current_line: Option<CurrentLine>,
}

impl TryFrom<RawEngineMessage> for Info {
    type Error = MessageTryFromRawMessageError<EngineMessageParameterPointer>;

    #[allow(clippy::too_many_lines)]
    fn try_from(
        raw_message: RawEngineMessage,
    ) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != EngineMessagePointer::Info {
            return Err(Self::Error::InvalidMessage);
        };

        let depth = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::Depth,
            ))
            .and_then(|s| {
                s.parse().ok().map(|depth| Depth {
                    depth,
                    selective_search_depth: raw_message
                        .parameters
                        .get(&EngineMessageParameterPointer::Info(
                            EngineMessageInfoParameterPointer::SelectiveSearchDepth,
                        ))
                        .and_then(|s| s.parse().ok()),
                })
            });

        let time = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::Time,
            ))
            .and_then(|s| s.parse().ok());

        let nodes = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::Nodes,
            ))
            .and_then(|s| s.parse().ok());

        let primary_variation = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::PrimaryVariation,
            ))
            .and_then(|s| s.parse().ok());

        let multi_primary_variation = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::MultiPrimaryVariation,
            ))
            .and_then(|s| s.parse().ok());

        let score = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::Score,
            ))
            .and_then(|s| {
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
                    mate_in.map(ScoreKind::MateIn),
                    |centipawns| Some(ScoreKind::Centipawns(centipawns))
                );

                let kind = kind?;

                Some(Score {
                    kind,
                    bound: if is_lowerbound && is_upperbound {
                        None
                    } else if is_lowerbound {
                        Some(ScoreBound::Lowerbound)
                    } else if is_upperbound {
                        Some(ScoreBound::Upperbound)
                    } else {
                        None
                    },
                })
            });

        let current_move = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::CurrentMove,
            ))
            .and_then(|s| s.parse().ok());

        let current_move_number = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::CurrentMoveNumber,
            ))
            .and_then(|s| s.parse().ok());

        let hash_full = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::HashFull,
            ))
            .and_then(|s| s.parse().ok());

        let nodes_per_second = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::NodesPerSecond,
            ))
            .and_then(|s| s.parse().ok());

        let table_base_hits = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::TableBaseHits,
            ))
            .and_then(|s| s.parse().ok());

        let shredder_base_hits = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::ShredderBaseHits,
            ))
            .and_then(|s| s.parse().ok());

        let cpu_load = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::CpuLoad,
            ))
            .and_then(|s| s.parse().ok());

        let string = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::String,
            ))
            .cloned();

        let refutation = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::Refutation,
            ))
            .and_then(|s| s.parse::<UciMoveList>().ok())
            .and_then(|move_list| {
                let refuted_move = move_list.0.first()?;
                let refutation = move_list.0.get(1..)?;

                Some(Refutation {
                    refuted_move: refuted_move.clone(),
                    refutation: UciMoveList(refutation.to_vec()),
                })
            });

        let current_line = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Info(
                EngineMessageInfoParameterPointer::CurrentLine,
            ))
            .and_then(|s| s.split_once(' '))
            .and_then(|(used_cpu, line)| {
                let Ok(used_cpu) = used_cpu.parse() else {
                    return None;
                };

                let Ok(line) = line.parse() else {
                    return None;
                };

                Some(CurrentLine {
                    used_cpu: Some(used_cpu),
                    line,
                })
            });

        Ok(Self {
            depth,
            time,
            nodes,
            primary_variation,
            multi_primary_variation,
            score,
            current_move,
            current_move_number,
            hash_full,
            nodes_per_second,
            table_base_hits,
            shredder_base_hits,
            cpu_load,
            string,
            refutation,
            current_line,
        })
    }
}

impl Display for Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("info")?;

        if let Some(depth) = &self.depth {
            write!(f, " depth {}", depth.depth)?;

            if let Some(selective_search_depth) = depth.selective_search_depth {
                write!(f, " seldepth {selective_search_depth}")?;
            }
        }

        if let Some(time) = self.time {
            write!(f, " time {time}")?;
        }

        if let Some(nodes) = self.nodes {
            write!(f, " nodes {nodes}")?;
        }

        if let Some(primary_variation) = &self.primary_variation {
            write!(f, " pv {primary_variation}")?;
        }

        if let Some(multi_primary_variation) = self.multi_primary_variation {
            write!(f, " multipv {multi_primary_variation}")?;
        }

        if let Some(score) = &self.score {
            f.write_str(" score")?;

            match score.kind {
                ScoreKind::Centipawns(centipawns) => write!(f, " cp {centipawns}")?,
                ScoreKind::MateIn(mate_in) => write!(f, " mate {mate_in}")?,
            }

            match score.bound {
                Some(ScoreBound::Upperbound) => {
                    f.write_str(" upperbound")?;
                }
                Some(ScoreBound::Lowerbound) => {
                    f.write_str(" lowerbound")?;
                }
                None => {}
            }
        }

        if let Some(current_move) = &self.current_move {
            write!(f, " currmove {current_move}")?;
        }

        if let Some(current_move_number) = self.current_move_number {
            write!(f, " currmovenumber {current_move_number}")?;
        }

        if let Some(hash_full) = self.hash_full {
            write!(f, " hashfull {hash_full}")?;
        }

        if let Some(nodes_per_second) = self.nodes_per_second {
            write!(f, " nps {nodes_per_second}")?;
        }

        if let Some(table_base_hits) = self.table_base_hits {
            write!(f, " tbhits {table_base_hits}")?;
        }

        if let Some(shredder_base_hits) = self.shredder_base_hits {
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
            f.write_str(&refutation.refutation.to_string())?;
        }

        if let Some(current_line) = &self.current_line {
            f.write_str(" currline")?;

            if let Some(used_cpu) = current_line.used_cpu {
                f.write_char(' ')?;
                f.write_str(&used_cpu.to_string())?;
            }

            f.write_char(' ')?;
            f.write_str(&current_line.line.to_string())?;
        }

        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::num::NonZeroUsize;
    use std::str::FromStr;
    use crate::messages::engine::{EngineMessage, info::{CurrentLine, Depth, Refutation, Score, ScoreBound}};
    use crate::{UciMoveList};
    use super::Info;
    use shakmaty::uci::UciMove;
    use pretty_assertions::assert_eq;
    use shakmaty::Color;
    use crate::messages::info::{Centipawns, ScoreKind, ScoreKindStandardized};
    
    #[test]
    fn score_kind_standardize() {
        assert_eq!(
            ScoreKind::Centipawns(-20).standardized(Color::White),
            ScoreKindStandardized::Centipawns(Centipawns::Black(NonZeroUsize::new(20).unwrap()))
        );
        assert_eq!(
            ScoreKind::Centipawns(-15).standardized(Color::Black),
            ScoreKindStandardized::Centipawns(Centipawns::White(NonZeroUsize::new(15).unwrap()))
        );
        assert_eq!(
            ScoreKind::Centipawns(10).standardized(Color::White),
            ScoreKindStandardized::Centipawns(Centipawns::White(NonZeroUsize::new(10).unwrap()))
        );
        assert_eq!(
            ScoreKind::Centipawns(5).standardized(Color::Black),
            ScoreKindStandardized::Centipawns(Centipawns::Black(NonZeroUsize::new(5).unwrap()))
        );
    }

    #[test]
    fn to_from_str() {
        let repr = EngineMessage::Info(Box::new(Info {
            depth: Some(Depth {
                depth: 20,
                selective_search_depth: Some(31)
            }),
            time: Some(12),
            nodes: Some(4),
            primary_variation: Some(UciMoveList(vec![UciMove::from_ascii(b"e2e4").unwrap(), UciMove::from_ascii(b"c7c5").unwrap()])),
            multi_primary_variation: Some(1),
            score: Some(Score {
                kind: ScoreKind::Centipawns(22),
                bound: Some(ScoreBound::Lowerbound),
            }),
            current_move: Some(UciMove::from_ascii(b"e2e4").unwrap()),
            current_move_number: None,
            hash_full: None,
            nodes_per_second: None,
            table_base_hits: Some(2),
            shredder_base_hits: None,
            cpu_load: None,
            string: Some("blabla".to_string()),
            refutation: Some(Refutation {
                refuted_move: UciMove::from_ascii(b"g2g4").unwrap(),
                refutation: UciMoveList(vec![UciMove::from_ascii(b"d7d5").unwrap(), UciMove::from_ascii(b"f1g2").unwrap()]),
            }),
            current_line: Some(CurrentLine {
                used_cpu: Some(1),
                line: UciMoveList(vec![UciMove::from_ascii(b"e2e4").unwrap(), UciMove::from_ascii(b"c7c5").unwrap()]),
            }),
        }));
        let str_repr = "info depth 20 seldepth 31 time 12 nodes 4 pv e2e4 c7c5 multipv 1 score cp 22 lowerbound currmove e2e4 tbhits 2 string blabla refutation g2g4 d7d5 f1g2 currline 1 e2e4 c7c5\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(EngineMessage::from_str(str_repr), Ok(repr));
    }
}