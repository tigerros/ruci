dry_mods::mods! {
    mod pub use id, best_move, copy_protection, info, option, registration;
}

use std::fmt::{Display, Formatter};
use crate::{define_message_enum, MessageTryFromRawUciMessageError, RawUciMessage};
use crate::UciMoveList;

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
        %%[parameters = [(Name, "name"), (Type, "type"), (Default, "default"), (Min, "min"), (Max, "max"), (Variation, "var")]]
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
                    )).and_then(|p| p.some());
                let author =
                    raw_uci_message
                        .parameters
                        .get(&EngineToGuiMessageParameterPointer::Id(
                            EngineToGuiMessageIdParameterPointer::Author,
                        )).and_then(|p| p.some());

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
                    )).and_then(|p| p.some())
                    .and_then(|s| {
                        s.parse().ok().map(|depth| InfoMessageDepthField {
                            depth,
                            selective_search_depth: raw_uci_message
                                .parameters
                                .get(&EngineToGuiMessageParameterPointer::Info(
                                    EngineToGuiMessageInfoParameterPointer::SelectiveSearchDepth,
                                )).and_then(|p| p.some())
                                .and_then(|s| s.parse().ok()),
                        })
                    });

                let time = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Time,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let nodes = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Nodes,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let primary_variation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::PrimaryVariation,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let multi_primary_variation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::MultiPrimaryVariation,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let score = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Score,
                    )).and_then(|p| p.some())
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
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let current_move_number = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::CurrentMoveNumber,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let hash_full = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::HashFull,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let nodes_per_second = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::NodesPerSecond,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let table_base_hits = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::TableBaseHits,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let shredder_base_hits = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::ShredderBaseHits,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let cpu_load = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::CpuLoad,
                    )).and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                let string = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::String,
                    )).and_then(|p| p.some()).cloned();

                let refutation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Refutation,
                    )).and_then(|p| p.some())
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
                    )).and_then(|p| p.some())
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
                return Ok(Self::Option(OptionMessage {
                    name: "".to_string(),
                    r#type: OptionMessageTypeField::Check,
                    default: None,
                    min: None,
                    max: None,
                    var: None,
                }));
            },
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
                    .get(&EngineToGuiMessageParameterPointer::BestMove(EngineToGuiMessageBestMoveParameterPointer::Ponder))
                    .and_then(|p| p.some())
                    .and_then(|s| s.parse().ok());

                return Ok(Self::BestMove(BestMoveMessage {
                    r#move,
                    ponder,
                }));
            },
            EngineToGuiMessagePointer::CopyProtection => {
                let Ok(kind) = value.parse() else {
                    return Err(Self::Error::ValueError);
                };

                return Ok(Self::CopyProtection(kind));
            },
            EngineToGuiMessagePointer::Registration => {
                let Ok(kind) = value.parse() else {
                    return Err(Self::Error::ValueError);
                };

                return Ok(Self::Registration(kind));
            },
            _ => {},
        }

        Err(Self::Error::ValueError)
    }
}

impl Display for EngineToGuiMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}