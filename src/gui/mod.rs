dry_mods::mods! {
    mod pub use go, register, set_option, position, debug;
}

use crate::dev_macros::define_message;
use crate::errors::MessageParseError;
use crate::parsing;
use core::str::FromStr;
use pointers::MessagePointer;

define_message! {
    /// A message sent from the GUI to the engine.
    enum Gui {
        =[custom]
        Debug(Debug),
        %[parameters = [Name, Value]]
        SetOption(SetOption),
        %[parameters = [Name, Code]]
        Register(Register),
        %[parameters = [Fen, Moves, StartPos]]
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

impl FromStr for Message {
    type Err = MessageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (message_pointer, parts) =
            parsing::collect_any_message::<MessagePointer>("GUI UCI message", s)?;

        match message_pointer {
            MessagePointer::Debug => Ok(Debug::from_str_parts_message_assumed(parts)?.into()),
            MessagePointer::SetOption => {
                Ok(SetOption::from_str_parts_message_assumed(parts)?.into())
            }
            MessagePointer::Register => Ok(Register::from_str_parts_message_assumed(parts)?.into()),
            MessagePointer::Position => Ok(Position::from_str_parts_message_assumed(parts).into()),
            MessagePointer::Go => Ok(Go::from_str_parts_message_assumed(parts).into()),
            MessagePointer::Uci => Ok(Uci.into()),
            MessagePointer::IsReady => Ok(IsReady.into()),
            MessagePointer::UciNewGame => Ok(UciNewGame.into()),
            MessagePointer::Stop => Ok(Stop.into()),
            MessagePointer::PonderHit => Ok(PonderHit.into()),
            MessagePointer::Quit => Ok(Quit.into()),
        }
    }
}
