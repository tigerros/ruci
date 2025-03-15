dry_mods::mods! {
    mod pub use go, register, set_option, position, debug;
}

use crate::dev_macros::define_message;

define_message! {
    /// A message sent from the GUI to the engine.
    enum Gui {
        =[custom]
        Debug(Debug),
        %[parameters = [Name, Value]]
        SetOption(SetOption),
        %[parameters = [Name, Code]]
        Register(Register),
        %[parameters = [Fen, Moves]]
        Position(Position),
        %[parameters = [SearchMoves, Ponder, WTime, BTime, WInc, BInc, MovesToGo, Depth, Nodes, Mate, MoveTime, Infinite]]
        Go(Go)
        =[empty]
        /// Tells the engine to use UCI.
        Uci,
        /// Asks the engine if it's ready.
        ///
        /// Once ready, engine replies with [`ReadyOk`](crate::engine::ReadyOk).
        IsReady,
        /// Tells the engine to start a new game with the next [`Position`](super::Position) or [`Go`](super::Go).
        UciNewGame,
        /// Tells the engine to stop calculating.
        Stop,
        PonderHit,
        /// Tells the engine to stop everything.
        Quit
    }
}
