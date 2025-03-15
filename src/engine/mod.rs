dry_mods::mods! {
    mod pub use id, best_move, copy_protection, registration, info, option;
}

extern crate alloc;

use crate::dev_macros::define_message;
use alloc::boxed::Box;

define_message! {
    /// A message sent from the engine to the GUI.
    enum Engine {
        =[custom]
        %[parameters = [Name, Author]]
        Id(Id),
        %[parameters = [Ponder]]
        BestMove(BestMove),
        CopyProtection(CopyProtection),
        Registration(Registration),
        %[parameters = [Depth, SelDepth, Time, Nodes, PV, MultiPV, Score, CurrMove, CurrMoveNumber, HashFull, Nps, TbHits, SbHits, CpuLoad, String, Refutation, CurrLine]]
        Info(Box<Info>),
        %[parameters = [Name, Type, Default, Min, Max, Var]]
        Option(Option)
        =[empty]
        /// The engine is ready for UCI.
        ///
        /// Sent after [`Uci`](crate::gui::Uci).
        UciOk,
        /// The engine is ready to accept new commands.
        ReadyOk
    }
}
