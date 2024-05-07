use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroUsize;
use crate::messages::GuiMessage;
use crate::{MessageTryFromRawUciMessageError, RawUciMessage, UciMoveList};
use crate::messages::gui::{GuiMessageGoParameterPointer, GuiMessageParameterPointer, GuiMessagePointer};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#gui-go>
pub struct GoMessage {
    /// <https://backscattering.de/chess/uci/#gui-go-searchmoves>
    pub search_moves: Option<UciMoveList>,
    /// <https://backscattering.de/chess/uci/#gui-go-ponder>
    pub ponder: bool,
    /// <https://backscattering.de/chess/uci/#gui-go-wtime>
    pub white_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-btime>
    pub black_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-winc>
    pub white_increment: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-binc>
    pub black_increment: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-movestogo>
    pub moves_to_go: Option<NonZeroUsize>,
    /// <https://backscattering.de/chess/uci/#gui-go-depth>
    pub depth: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-nodes>
    pub nodes: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-mate>
    pub mate: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-movetime>
    pub move_time: Option<usize>,
    /// <https://backscattering.de/chess/uci/#gui-go-infinite>
    pub infinite: bool,
}

impl TryFrom<RawUciMessage<GuiMessage>> for GoMessage {
    type Error = MessageTryFromRawUciMessageError<GuiMessageParameterPointer>;

    fn try_from(raw_uci_message: RawUciMessage<GuiMessage>) -> Result<Self, Self::Error> {
        if raw_uci_message.message_pointer != GuiMessagePointer::Go {
            return Err(Self::Error::InvalidMessage);
        };

        let search_moves = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::SearchMoves,
            ))
            .and_then(|s| s.parse().ok());

        let ponder =
            raw_uci_message
                .void_parameters
                .contains(&GuiMessageParameterPointer::Go(
                    GuiMessageGoParameterPointer::Ponder,
                ));

        let white_time = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::WhiteTime,
            ))
            .and_then(|s| s.parse().ok());

        let black_time = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::BlackTime,
            ))
            .and_then(|s| s.parse().ok());

        let white_increment = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::WhiteIncrement,
            ))
            .and_then(|s| s.parse().ok());

        let black_increment = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::BlackIncrement,
            ))
            .and_then(|s| s.parse().ok());

        let moves_to_go = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::MovesToGo,
            ))
            .and_then(|s| s.parse().ok());

        let depth = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::Depth,
            ))
            .and_then(|s| s.parse().ok());

        let nodes = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::Nodes,
            ))
            .and_then(|s| s.parse().ok());

        let mate = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::Mate,
            ))
            .and_then(|s| s.parse().ok());

        let move_time = raw_uci_message
            .parameters
            .get(&GuiMessageParameterPointer::Go(
                GuiMessageGoParameterPointer::MoveTime,
            ))
            .and_then(|s| s.parse().ok());

        let infinite =
            raw_uci_message
                .void_parameters
                .contains(&GuiMessageParameterPointer::Go(
                    GuiMessageGoParameterPointer::Infinite,
                ));

        Ok(Self {
            search_moves,
            ponder,
            white_time,
            black_time,
            white_increment,
            black_increment,
            moves_to_go,
            depth,
            nodes,
            mate,
            move_time,
            infinite,
        })
    }
}

impl Display for GoMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("go")?;

        if let Some(search_moves) = &self.search_moves {
            write!(f, " searchmoves {}", &search_moves)?;
        }

        if self.ponder {
            f.write_str(" ponder")?;
        }

        if let Some(white_time) = self.white_time {
            write!(f, " wtime {white_time}")?;
        }

        if let Some(black_time) = self.black_time {
            write!(f, " btime {black_time}")?;
        }

        if let Some(white_increment) = self.white_increment {
            write!(f, " winc {white_increment}")?;
        }

        if let Some(black_increment) = self.black_increment {
            write!(f, " binc {black_increment}")?;
        }

        if let Some(moves_to_go) = self.moves_to_go {
            write!(f, " moves_to_go {moves_to_go}")?;
        }

        if let Some(depth) = self.depth {
            write!(f, " depth {depth}")?;
        }

        if let Some(nodes) = self.nodes {
            write!(f, " nodes {nodes}")?;
        }

        if let Some(mate) = self.mate {
            write!(f, " mate {mate}")?;
        }

        if let Some(move_time) = self.move_time {
            write!(f, " movetime {move_time}")?;
        }

        if self.infinite {
            f.write_str(" infinite")?;
        }
        
        f.write_char('\n')
    }
}