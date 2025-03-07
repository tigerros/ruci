dry_mods::mods! {
    mod pub use go, register, set_option, set_position;
}

use crate::define_message::define_message;
use std::fmt::{Display, Formatter};

define_message! {
    /// A message sent from the GUI to the engine.
    enum Gui {
        /// <https://backscattering.de/chess/uci/#gui-uci>
        %["uci"]
        UseUci,
        /// <https://backscattering.de/chess/uci/#gui-debug>
        %["debug"]
        Debug(%bool),
        /// <https://backscattering.de/chess/uci/#gui-isready>
        %["isready"]
        IsReady,
        /// <https://backscattering.de/chess/uci/#gui-setoption>
        %["setoption"]
        %[parameters = [Name: "name", Value: "value"]]
        SetOption(%SetOption),
        /// <https://backscattering.de/chess/uci/#gui-register>
        %["register"]
        %[parameters = [Name: "name", Code: "code"]]
        Register(%Register),
        /// <https://backscattering.de/chess/uci/#gui-ucinewgame>
        %["ucinewgame"]
        UciNewGame,
        /// <https://backscattering.de/chess/uci/#gui-position>
        %["position"]
        %[parameters = [Fen: "fen", Moves: "moves"]]
        SetPosition(%SetPosition),
        /// <https://backscattering.de/chess/uci/#gui-go>
        %["go"]
        %[parameters = [SearchMoves: "searchmoves", **Ponder: "ponder", WhiteTime: "wtime", BlackTime: "btime", WhiteIncrement: "winc", BlackIncrement: "binc", MovesToGo: "movestogo", Depth: "depth", Nodes: "nodes", Mate: "mate", MoveTime: "movetime", **Infinite: "infinite"]]
        Go(%Go),
        /// <https://backscattering.de/chess/uci/#gui-stop>
        %["stop"]
        Stop,
        /// <https://backscattering.de/chess/uci/#gui-ponderhit>
        %["ponderhit"]
        PonderHit,
        /// <https://backscattering.de/chess/uci/#gui-quit>
        %["quit"]
        Quit
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseUci => f.write_str("uci\n"),
            Self::Debug(value) => writeln!(f, "debug {}", if *value { "on" } else { "off" }),
            Self::IsReady => f.write_str("isready\n"),
            Self::SetOption(m) => m.fmt(f),
            Self::Register(m) => m.fmt(f),
            Self::UciNewGame => f.write_str("ucinewgame\n"),
            Self::SetPosition(m) => m.fmt(f),
            Self::Go(m) => m.fmt(f),
            Self::Stop => f.write_str("stop\n"),
            Self::PonderHit => f.write_str("ponderhit\n"),
            Self::Quit => f.write_str("quit\n"),
        }
    }
}
