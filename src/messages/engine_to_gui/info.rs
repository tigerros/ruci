use crate::UciMoveList;
use shakmaty::uci::Uci as UciMove;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-info-depth>
pub struct InfoMessageDepthField {
    /// <https://backscattering.de/chess/uci/#engine-info-depth>
    pub depth: usize,
    /// <https://backscattering.de/chess/uci/#engine-info-seldepth>
    pub selective_search_depth: Option<usize>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InfoMessageScoreFieldBound {
    /// If neither [`lowerbound`](https://backscattering.de/chess/uci/#engine-info-score-lowerbound) nor [`upperbound`](https://backscattering.de/chess/uci/#engine-info-score-upperbound) is present.
    Unspecified,
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    Lowerbound,
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    Upperbound,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-info-score>
pub struct InfoMessageScoreField {
    /// <https://backscattering.de/chess/uci/#engine-info-score-cp>
    pub centipawns: Option<isize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score-mate>
    pub mate_in: Option<isize>,
    /// <https://backscattering.de/chess/uci/#engine-info-score-lowerbound>
    /// <https://backscattering.de/chess/uci/#engine-info-score-upperbound>
    pub bound: InfoMessageScoreFieldBound,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-info-refutation>
pub struct InfoMessageRefutationField {
    pub refuted_move: UciMove,
    pub refutation: UciMoveList,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-info-currline>
pub struct InfoMessageCurrentLineField {
    pub used_cpu: Option<usize>,
    pub line: UciMoveList,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::messages::engine_to_gui::{EngineToGuiMessage, InfoMessageCurrentLineField, InfoMessageDepthField, InfoMessageRefutationField, InfoMessageScoreField, InfoMessageScoreFieldBound};
    use crate::{Message, UciMoveList};
    use super::InfoMessage;
    use shakmaty::uci::Uci as UciMove;
    use pretty_assertions::assert_eq;
    fn repr() -> (EngineToGuiMessage, String) {
        (EngineToGuiMessage::Info(Box::new(InfoMessage {
            depth: Some(InfoMessageDepthField {
                depth: 20,
                selective_search_depth: Some(31)
            }),
            time: Some(12),
            nodes: Some(4),
            primary_variation: Some(UciMoveList(vec![UciMove::from_ascii(b"e2e4").unwrap(), UciMove::from_ascii(b"c7c5").unwrap()])),
            multi_primary_variation: Some(1),
            score: Some(InfoMessageScoreField {
                centipawns: Some(22),
                mate_in: None,
                bound: InfoMessageScoreFieldBound::Lowerbound,
            }),
            current_move: Some(UciMove::from_ascii(b"e2e4").unwrap()),
            current_move_number: None,
            hash_full: None,
            nodes_per_second: None,
            table_base_hits: Some(2),
            shredder_base_hits: None,
            cpu_load: None,
            string: Some("blabla".to_string()),
            refutation: Some(InfoMessageRefutationField {
                refuted_move: UciMove::from_ascii(b"g2g4").unwrap(),
                refutation: UciMoveList(vec![UciMove::from_ascii(b"d7d5").unwrap(), UciMove::from_ascii(b"f1g2").unwrap()]),
            }),
            current_line: Some(InfoMessageCurrentLineField {
                used_cpu: Some(1),
                line: UciMoveList(vec![UciMove::from_ascii(b"e2e4").unwrap(), UciMove::from_ascii(b"c7c5").unwrap()]),
            }),
        })), "info depth 20 seldepth 31 time 12 nodes 4 pv e2e4 c7c5 multipv 1 score cp 22 lowerbound currmove e2e4 tbhits 2 string blabla refutation g2g4 d7d5 f1g2 currline 1 e2e4 c7c5\n".to_string())
    }

    #[test]
    fn to_string() {
        let repr = repr();
        
        assert_eq!(repr.0.to_string(), repr.1);
    }
    
    #[test]
    fn from_string() {
        let repr = repr();
        
        assert_eq!(EngineToGuiMessage::from_str(&repr.1), Ok(repr.0));
    }
}