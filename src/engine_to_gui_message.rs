use crate::{define_message_enum, RawUciMessage};
use crate::UciMoveList;
use shakmaty::uci::Uci as UciMove;

#[derive(Debug)]
/// <https://backscattering.de/chess/uci/#engine-id>
pub enum IdMessageKind {
    /// <https://backscattering.de/chess/uci/#engine-id-name>
    Name(String),
    /// <https://backscattering.de/chess/uci/#engine-id-author>
    Author(String),
    NameAndAuthor(String, String),
}

#[derive(Debug)]
pub struct BestMoveMessage {
    pub r#move: UciMove,
    pub ponder: Option<UciMove>,
}

#[derive(Debug, Copy, Clone)]
pub enum CopyProtectionMessageKind {
    Ok,
    Error,
}

#[derive(Debug, Copy, Clone)]
pub enum RegistrationMessageKind {
    Checking,
    Ok,
    Error,
}

#[derive(Debug)]
/// <https://backscattering.de/chess/uci/#engine-info-depth>
pub struct InfoMessageDepthField {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: usize,
    /// <https://backscattering.de/chess/uci/#engine-info-seldepth>
    pub selective_search_depth: Option<usize>,
}

#[derive(Debug, Copy, Clone)]
pub enum InfoMessageScoreFieldBound {
    /// If neither `lowerbound` nor `upperbound` is present.
    Unspecified,
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    Lowerbound,
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    Upperbound,
}

#[derive(Debug)]
/// <https://backscattering.de/chess/uci/#engine-info-score>
pub struct InfoMessageScoreField {
    /// <https://backscattering.de/chess/uci/#engine-info-score-cp>
    pub centipawns: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score-mate>
    pub mate_in: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    pub bound: InfoMessageScoreFieldBound,
}

#[derive(Debug)]
/// <https://backscattering.de/chess/uci/#engine-info-refutation>
pub struct InfoMessageRefutationField {
    pub refuted_move: UciMove,
    pub refutation: UciMoveList,
}

#[derive(Debug)]
/// <https://backscattering.de/chess/uci/#engine-info-currline>
pub struct InfoMessageCurrentLineField {
    pub used_cpu: Option<usize>,
    pub line: UciMoveList,
}

#[derive(Debug)]
pub struct InfoMessage {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: Option<InfoMessageDepthField>,
    /// <https://backscattering.de/chess/uci/#engine-info-time>
    pub time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-nodes>
    pub nodes: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-pv>
    pub primary_variation: Option<UciMoveList>,
    /// <https://backscattering.de/chess/uci/#engine-info-multipv>
    pub multi_primary_variation: Option<usize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score>
    pub score: Option<InfoMessageScoreField>,
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
    pub refutation: Option<InfoMessageRefutationField>,
    /// <https://backscattering.de/chess/uci/#engine-info-currline>
    pub current_line: Option<InfoMessageCurrentLineField>,
}

#[derive(Debug, Copy, Clone)]
/// <https://backscattering.de/chess/uci/#engine-option-type>
pub enum OptionMessageTypeField {
    /// <https://backscattering.de/chess/uci/#engine-option-type-check>
    Check,
    /// <https://backscattering.de/chess/uci/#engine-option-type-spin>
    Spin,
    /// <https://backscattering.de/chess/uci/#engine-option-type-combo>
    Combo,
    /// <https://backscattering.de/chess/uci/#engine-option-type-button>
    Button,
    /// <https://backscattering.de/chess/uci/#engine-option-type-string>
    String,
}

#[derive(Debug)]
pub struct OptionMessage {
    /// <https://backscattering.de/chess/uci/#engine-option-name>
    pub name: String,
    /// <https://backscattering.de/chess/uci/#engine-option-type>
    pub r#type: OptionMessageTypeField,
    /// <https://backscattering.de/chess/uci/#engine-option-default>
    pub default: Option<String>,
    /// <https://backscattering.de/chess/uci/#engine-option-min>
    pub min: Option<isize>,
    /// <https://backscattering.de/chess/uci/#engine-option-max>
    pub max: Option<isize>,
    /// <https://backscattering.de/chess/uci/#engine-option-var>
    pub var: Option<String>,
}

define_message_enum! {
    #[derive(Debug)]
    pub enum EngineToGuiMessage {
        /// <https://backscattering.de/chess/uci/#engine-id>
        %["id"]
        %%[parameters = [(Name, "name"), (Author, "author")]]
        Id(IdMessageKind),
        /// <https://backscattering.de/chess/uci/#engine-uciok>
        %["uciok"]
        UciOk,
        /// <https://backscattering.de/chess/uci/#engine-readyok>
        %["readyok"]
        ReadyOk,
        /// <https://backscattering.de/chess/uci/#engine-bestmove>
        %["bestmove"]
        %%[parameters = [(Ponder, "ponder")]]
        BestMove(BestMoveMessage),
        /// <https://backscattering.de/chess/uci/#engine-copyprotection>
        %["copyprotection"]
        CopyProtection(CopyProtectionMessageKind),
        /// <https://backscattering.de/chess/uci/#engine-registration>
        %["registration"]
        Registration(RegistrationMessageKind),
        /// <https://backscattering.de/chess/uci/#engine-info>
        %["info"]
        %%[parameters = [(Depth, "depth"), (SelectiveSearchDepth, "seldepth"), (Time, "time"), (Nodes, "nodes"), (PrimaryVariation, "pv"), (MultiPrimaryVariation, "multipv"), (Score, "score"), (CurrentMove, "currmove"), (CurrentMoveNumber, "currmovenumber"), (HashFull, "hashfull"), (NodesPerSecond, "nps"), (TableBaseHits, "tbhits"), (ShredderBaseHits, "sbhits"), (CpuLoad, "cpuload"), (String, "string"), (Refutation, "refutation"), (CurrentLine, "currline")]]
        Info(Box<InfoMessage>),
        /// <https://backscattering.de/chess/uci/#engine-option>
        %["option"]
        %%[parameters = [(Name, "name"), (Type, "type"), (Default, "default"), (Min, "min"), (Max, "max"), (Variation, "var")]]
        Option(OptionMessage)
    }
}

#[derive(Debug)]
pub enum EngineToGuiMessageTryFromRawUciMessageError {
    // TODO: Better errors
    NeedParameters,
    NeedValue,
}

impl TryFrom<RawUciMessage<EngineToGuiMessagePointer, EngineToGuiMessageParameterPointer>>
    for EngineToGuiMessage
{
    type Error = EngineToGuiMessageTryFromRawUciMessageError;

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
                    ));
                let author =
                    raw_uci_message
                        .parameters
                        .get(&EngineToGuiMessageParameterPointer::Id(
                            EngineToGuiMessageIdParameterPointer::Author,
                        ));

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

                return Err(Self::Error::NeedParameters);
            }
            EngineToGuiMessagePointer::Info => {
                let depth = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Depth,
                    ))
                    .and_then(|s| {
                        s.parse().ok().map(|depth| InfoMessageDepthField {
                            depth,
                            selective_search_depth: raw_uci_message
                                .parameters
                                .get(&EngineToGuiMessageParameterPointer::Info(
                                    EngineToGuiMessageInfoParameterPointer::SelectiveSearchDepth,
                                ))
                                .and_then(|s| s.parse().ok()),
                        })
                    });

                let time = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Time,
                    ))
                    .and_then(|s| s.parse().ok());

                let nodes = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Nodes,
                    ))
                    .and_then(|s| s.parse().ok());

                let primary_variation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::PrimaryVariation,
                    ))
                    .and_then(|s| s.parse().ok());

                let multi_primary_variation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::MultiPrimaryVariation,
                    ))
                    .and_then(|s| s.parse().ok());

                let score = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Score,
                    ))
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
                    .and_then(|s| s.parse().ok());

                let current_move_number = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::CurrentMoveNumber,
                    ))
                    .and_then(|s| s.parse().ok());

                let hash_full = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::HashFull,
                    ))
                    .and_then(|s| s.parse().ok());

                let nodes_per_second = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::NodesPerSecond,
                    ))
                    .and_then(|s| s.parse().ok());

                let table_base_hits = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::TableBaseHits,
                    ))
                    .and_then(|s| s.parse().ok());

                let shredder_base_hits = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::ShredderBaseHits,
                    ))
                    .and_then(|s| s.parse().ok());

                let cpu_load = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::CpuLoad,
                    ))
                    .and_then(|s| s.parse().ok());

                let string = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::String,
                    ))
                    .cloned();

                let refutation = raw_uci_message
                    .parameters
                    .get(&EngineToGuiMessageParameterPointer::Info(
                        EngineToGuiMessageInfoParameterPointer::Refutation,
                    ))
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
            }
            _ => todo!(),
        }

        // Messages with parameters/values
    }
}