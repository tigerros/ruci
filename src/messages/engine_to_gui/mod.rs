dry_mods::mods! {
    mod pub use id, best_move, copy_protection, info, option, registration;
}

use crate::UciMoveList;
use crate::{define_message_enum, MessageTryFromRawUciMessageError, RawUciMessage};
use std::fmt::{Display, Formatter};

define_message_enum! {
    pub enum EngineToGuiMessage {
        /// <https://backscattering.de/chess/uci/#engine-id>
        %["id"]
        %%[parameters = [(Name, "name"), (Author, "author")]]
        Id(%IdMessageKind),
        /// <https://backscattering.de/chess/uci/#engine-uciok>
        %["uciok"]
        UciOk,
        /// <https://backscattering.de/chess/uci/#engine-readyok>
        %["readyok"]
        ReadyOk,
        /// <https://backscattering.de/chess/uci/#engine-bestmove>
        %["bestmove"]
        %%[parameters = [(Ponder, "ponder")]]
        BestMove(%BestMoveMessage),
        /// <https://backscattering.de/chess/uci/#engine-copyprotection>
        %["copyprotection"]
        CopyProtection(%CopyProtectionMessageKind),
        /// <https://backscattering.de/chess/uci/#engine-registration>
        %["registration"]
        Registration(%RegistrationMessageKind),
        /// <https://backscattering.de/chess/uci/#engine-info>
        %["info"]
        %%[parameters = [(Depth, "depth"), (SelectiveSearchDepth, "seldepth"), (Time, "time"), (Nodes, "nodes"), (PrimaryVariation, "pv"), (MultiPrimaryVariation, "multipv"), (Score, "score"), (CurrentMove, "currmove"), (CurrentMoveNumber, "currmovenumber"), (HashFull, "hashfull"), (NodesPerSecond, "nps"), (TableBaseHits, "tbhits"), (ShredderBaseHits, "sbhits"), (CpuLoad, "cpuload"), (String, "string"), (Refutation, "refutation"), (CurrentLine, "currline")]]
        Info(%Box<InfoMessage>),
        /// <https://backscattering.de/chess/uci/#engine-option>
        %["option"]
        %%[parameters = [(Name, "name"), (Type, "type"), (Default, "default"), (Min, "min"), (Max, "max"), (Var, "var")]]
        Option(%OptionMessage)
    }
}

impl TryFrom<RawUciMessage<EngineToGuiMessagePointer, EngineToGuiMessageParameterPointer>>
    for EngineToGuiMessage
{
    type Error = MessageTryFromRawUciMessageError;

    #[allow(clippy::too_many_lines)]
    fn try_from(
        raw_uci_message: RawUciMessage<
            EngineToGuiMessagePointer,
            EngineToGuiMessageParameterPointer,
        >,
    ) -> Result<Self, Self::Error> {
        // Parameter-less, value-less messages
        match raw_uci_message.message_pointer {
            EngineToGuiMessagePointer::UciOk => return Ok(Self::UciOk),
            EngineToGuiMessagePointer::ReadyOk => return Ok(Self::ReadyOk),
            _ => (),
        }

        // Value-less messages
        match raw_uci_message.message_pointer {
            EngineToGuiMessagePointer::Id => {
                let name = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Id(
                        EngineToGuiMessageIdParameterPointer::Name,
                    ))
                    .and_then(|p| p.some());
                let author = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Id(
                        EngineToGuiMessageIdParameterPointer::Author,
                    ))
                    .and_then(|p| p.some());

                if let Some(name) = name {
                    if let Some(author) = author {
                        return Ok(Self::Id(IdMessageKind::NameAndAuthor(
                            name.to_string(),
                            author.to_string(),
                        )));
                    }

                    return Ok(Self::Id(IdMessageKind::Name(name.to_string())));
                } else if let Some(author) = author {
                    return Ok(Self::Id(IdMessageKind::Author(author.to_string())));
                }

                return Err(Self::Error::ParameterError);
            }
            EngineToGuiMessagePointer::Info => {
                let depth = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Depth,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| {
                        s.parse().ok().map(|depth| InfoMessageDepthField {
                            depth,
                            selective_search_depth: raw_uci_message
                                .parameters
                                .get(&EngineToGuiMessageParameterPointer::Info(
                                    EngineToGuiMessageInfoParameterPointer::SelectiveSearchDepth,
                                ))
                                .and_then(|p| p.some())
                                .and_then(|s| s.parse().ok()),
                        })
                    });

                let time = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Time,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());
                
                println!("infotime: {:?}", 
                         raw_uci_message.parameters
                             .get(&EngineToGuiMessageParameterPointer::Info(EngineToGuiMessageInfoParameterPointer::Time)));

                let nodes = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Nodes,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let primary_variation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::PrimaryVariation,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let multi_primary_variation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::MultiPrimaryVariation,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let score = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Score,
                    ))
                    .and_then(|p| p.some())
                    .map(|s| {
                        let split = s.split(' ').collect::<Vec<_>>();
                        let mut split_iter = split.iter();
                        let centipawns_position = split_iter.position(|&part| part == "cp");
                        let mate_in_position = split_iter.position(|&part| part == "mate");
                        let is_lowerbound = split_iter.any(|&part| part == "lowerbound");
                        let is_upperbound = split_iter.any(|&part| part == "upperbound");

                        let centipawns = centipawns_position.and_then(|centipawns_position| {
                            split
                                .get(centipawns_position + 1)
                                .and_then(|s| s.parse().ok())
                        });

                        let mate_in = mate_in_position.and_then(|mate_in_position| {
                            split.get(mate_in_position + 1).and_then(|s| s.parse().ok())
                        });

                        InfoMessageScoreField {
                            centipawns,
                            mate_in,
                            bound: if is_lowerbound && is_upperbound {
                                InfoMessageScoreFieldBound::Unspecified
                            } else if is_lowerbound {
                                InfoMessageScoreFieldBound::Lowerbound
                            } else if is_upperbound {
                                InfoMessageScoreFieldBound::Upperbound
                            } else {
                                InfoMessageScoreFieldBound::Unspecified
                            },
                        }
                    });

                let current_move = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::CurrentMove,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let current_move_number = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::CurrentMoveNumber,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let hash_full = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::HashFull,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let nodes_per_second = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::NodesPerSecond,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let table_base_hits = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::TableBaseHits,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let shredder_base_hits = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::ShredderBaseHits,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let cpu_load = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::CpuLoad,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let string = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::String,
                    ))
                    .and_then(|p| p.some())
                    .cloned();

                let refutation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Refutation,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse::<UciMoveList>().ok())
                    .and_then(|move_list| {
                        let Some(refuted_move) = move_list.0.first() else {
                            return None;
                        };

                        let Some(refutation) = move_list.0.get(1..) else {
                            return None;
                        };

                        Some(InfoMessageRefutationField {
                            refuted_move: refuted_move.clone(),
                            refutation: UciMoveList(refutation.to_vec()),
                        })
                    });

                let current_line = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::CurrentLine,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.split_once(' '))
                    .and_then(|(used_cpu, line)| {
                        let Ok(used_cpu) = used_cpu.parse() else {
                            return None;
                        };

                        let Ok(line) = line.parse() else {
                            return None;
                        };

                        Some(InfoMessageCurrentLineField {
                            used_cpu: Some(used_cpu),
                            line,
                        })
                    });

                return Ok(Self::Info(Box::new(InfoMessage {
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
                })));
            }
            EngineToGuiMessagePointer::Option => {
                let Some(name) = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Option(
                        EngineToGuiMessageOptionParameterPointer::Name,
                    ))
                    .and_then(|p| p.some())
                    .cloned()
                else {
                    return Err(Self::Error::ParameterError);
                };

                let Some(r#type) = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Option(
                        EngineToGuiMessageOptionParameterPointer::Type,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok())
                else {
                    return Err(Self::Error::ParameterError);
                };

                let default = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Option(
                        EngineToGuiMessageOptionParameterPointer::Default,
                    ))
                    .and_then(|p| p.some())
                    .cloned();

                let min = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Option(
                        EngineToGuiMessageOptionParameterPointer::Min,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let max = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Option(
                        EngineToGuiMessageOptionParameterPointer::Max,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let var = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Option(
                        EngineToGuiMessageOptionParameterPointer::Var,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                return Ok(Self::Option(OptionMessage {
                    name,
                    r#type,
                    default,
                    min,
                    max,
                    var,
                }));
            }
            _ => {}
        }

        // Messages with parameters/values
        let Some(value) = raw_uci_message.value else {
            return Err(Self::Error::ValueError);
        };

        match raw_uci_message.message_pointer {
            EngineToGuiMessagePointer::BestMove => {
                let Ok(r#move) = value.parse() else {
                    return Err(Self::Error::ValueError);
                };

                let ponder = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::BestMove(
                        EngineToGuiMessageBestMoveParameterPointer::Ponder,
                    ))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                return Ok(Self::BestMove(BestMoveMessage { r#move, ponder }));
            }
            EngineToGuiMessagePointer::CopyProtection => {
                let Ok(kind) = value.parse() else {
                    return Err(Self::Error::ValueError);
                };

                return Ok(Self::CopyProtection(kind));
            }
            EngineToGuiMessagePointer::Registration => {
                let Ok(kind) = value.parse() else {
                    return Err(Self::Error::ValueError);
                };

                return Ok(Self::Registration(kind));
            }
            _ => {}
        }

        Err(Self::Error::ValueError)
    }
}

impl Display for EngineToGuiMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineToGuiMessage::Id(kind) => {
                f.write_str("id ")?;

                match kind {
                    IdMessageKind::Name(name) => write!(f, "name {name}"),
                    IdMessageKind::Author(author) => write!(f, "author {author}"),
                    IdMessageKind::NameAndAuthor(name, author) => write!(f, "name {name} author {author}"),
                }
            },
            EngineToGuiMessage::UciOk => f.write_str("uciok"),
            EngineToGuiMessage::ReadyOk => f.write_str("readyok"),
            EngineToGuiMessage::BestMove(message) => {
                write!(f, "bestmove {}", message.r#move)?;

                if let Some(ponder) = &message.ponder {
                    write!(f, " {ponder}")?;
                }

                Ok(())
            },
            EngineToGuiMessage::CopyProtection(kind) => write!(f, "copyprotection {kind}"),
            EngineToGuiMessage::Registration(kind) => write!(f, "registration {kind}"),
            EngineToGuiMessage::Info(info) => {
                f.write_str("info")?;

                if let Some(depth) = &info.depth {
                    write!(f, " depth {}", depth.depth)?;

                    if let Some(selective_search_depth) = depth.selective_search_depth {
                        write!(f, " seldepth {selective_search_depth}")?;
                    }
                }

                if let Some(time) = info.time {
                    write!(f, " time {time}")?;
                }

                if let Some(nodes) = info.nodes {
                    write!(f, " nodes {nodes}")?;
                }

                if let Some(primary_variation) = &info.primary_variation {
                    write!(f, " pv {primary_variation}")?;
                }

                if let Some(multi_primary_variation) = info.multi_primary_variation {
                    write!(f, " multipv {multi_primary_variation}")?;
                }

                if let Some(score) = &info.score {
                    if let Some(centipawns) = score.centipawns {
                        write!(f, " cp {centipawns}")?;
                    }

                    if let Some(mate_in) = score.mate_in {
                        write!(f, " matein {mate_in}")?;
                    }

                    match score.bound {
                        InfoMessageScoreFieldBound::Upperbound => f.write_str(" upperbound")?,
                        InfoMessageScoreFieldBound::Lowerbound => f.write_str(" lowerbound")?,
                        InfoMessageScoreFieldBound::Unspecified => {}
                    }
                }

                if let Some(current_move) = &info.current_move {
                    write!(f, " currmove {current_move}")?;
                }

                if let Some(current_move_number) = info.current_move_number {
                    write!(f, " currmovenumber {current_move_number}")?;
                }

                if let Some(hash_full) = info.hash_full {
                    write!(f, " hashfull {hash_full}")?;
                }

                if let Some(nodes_per_second) = info.nodes_per_second {
                    write!(f, " nps {nodes_per_second}")?;
                }

                if let Some(table_base_hits) = info.table_base_hits {
                    write!(f, " tbhits {table_base_hits}")?;
                }

                if let Some(shredder_base_hits) = info.shredder_base_hits {
                    write!(f, " sbhits {shredder_base_hits}")?;
                }

                if let Some(cpu_load) = info.cpu_load {
                    write!(f, " cpuload {cpu_load}")?;
                }

                if let Some(string) = &info.string {
                    write!(f, " string {string}")?;
                }

                if let Some(refutation) = &info.refutation {
                    write!(f, " refutation {} ", refutation.refuted_move)?;
                    f.write_str(&refutation.refutation.to_string())?;
                }

                if let Some(current_line) = &info.current_line {
                    f.write_str("currline ")?;

                    if let Some(used_cpu) = current_line.used_cpu {
                        f.write_str(&used_cpu.to_string())?;
                    }

                    f.write_str(&current_line.line.to_string())?;
                }

                Ok(())
            },
            EngineToGuiMessage::Option(option) => {
                write!(f, "option name {} type {}", option.name, option.r#type);

                if let Some(default) = &option.default {
                    write!(f, " default {default}")?;
                }

                if let Some(min) = option.min {
                    write!(f, " min {min}")?;
                }

                if let Some(max) = option.max {
                    write!(f, " max {max}")?;
                }

                if let Some(var) = &option.var {
                    write!(f, " var {var}")?;
                }
                
                Ok(())
            },
        }
    }
}