dry_mods::mods! {
    mod pub use id, best_move, copy_protection, registration, info, option;
}

extern crate alloc;

use crate::define_message::define_message;
use alloc::boxed::Box;
use core::fmt::{Display, Formatter};

define_message! {
    /// A message sent from the engine to the GUI.
    enum Engine {
        /// <https://backscattering.de/chess/uci/#engine-id>
        %["id"]
        %[parameters = [Name: "name", Author: "author"]]
        Id(%Id),
        /// <https://backscattering.de/chess/uci/#engine-uciok>
        %["uciok"]
        UciOk,
        /// <https://backscattering.de/chess/uci/#engine-readyok>
        %["readyok"]
        ReadyOk,
        /// <https://backscattering.de/chess/uci/#engine-bestmove>
        %["bestmove"]
        %[parameters = [Ponder: "ponder"]]
        BestMove(%BestMove),
        /// <https://backscattering.de/chess/uci/#engine-copyprotection>
        %["copyprotection"]
        CopyProtection(%CopyProtection),
        /// <https://backscattering.de/chess/uci/#engine-registration>
        %["registration"]
        Registration(%Registration),
        /// <https://backscattering.de/chess/uci/#engine-info>
        %["info"]
        %[parameters = [Depth: "depth", SelectiveSearchDepth: "seldepth", Time: "time", Nodes: "nodes", PrimaryVariation: "pv", MultiPrimaryVariation: "multipv", Score: "score", CurrentMove: "currmove", CurrentMoveNumber: "currmovenumber", HashFull: "hashfull", NodesPerSecond: "nps", TableBaseHits: "tbhits", ShredderBaseHits: "sbhits", CpuLoad: "cpuload", String: "string", Refutation: "refutation", CurrentLine: "currline"]]
        Info(%Box<Info>),
        /// <https://backscattering.de/chess/uci/#engine-option>
        %["option"]
        %[parameters = [Name: "name", Type: "type", Default: "default", Min: "min", Max: "max", Var: "var"]]
        Option(%Option)
    }
}

impl Display for Message {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Id(m) => m.fmt(f),
            Self::UciOk => f.write_str("uciok\n"),
            Self::ReadyOk => f.write_str("readyok\n"),
            Self::BestMove(m) => m.fmt(f),
            Self::CopyProtection(m) => m.fmt(f),
            Self::Registration(m) => m.fmt(f),
            Self::Info(m) => m.fmt(f),
            Self::Option(m) => m.fmt(f),
        }
    }
}
