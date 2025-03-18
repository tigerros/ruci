dry_mods::mods! {
    mod pub use id, best_move, copy_protection, registration, info, option;
}

extern crate alloc;

use crate::dev_macros::define_message;
use crate::engine::pointers::MessagePointer;
use crate::errors::MessageParseError;
use crate::parsing;
use alloc::boxed::Box;
use core::str::FromStr;

define_message! {
    /// A message sent from the engine to the GUI.
    enum Engine {
        =[custom]
        %[parameters = [Name, Author]]
        Id(Id),
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

impl FromStr for Message {
    type Err = MessageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (message_pointer, parts) =
            parsing::collect_any_message::<MessagePointer>("engine UCI message", s)?;

        match message_pointer {
            MessagePointer::Id => Ok(Id::from_str_parts_message_assumed(parts)?.into()),
            MessagePointer::BestMove => Ok(BestMove::from_str_parts_message_assumed(parts).into()),
            MessagePointer::CopyProtection => {
                Ok(CopyProtection::from_str_parts_message_assumed(parts)?.into())
            }
            MessagePointer::Registration => {
                Ok(Registration::from_str_parts_message_assumed(parts)?.into())
            }
            MessagePointer::Info => Ok(Info::from_str_parts_message_assumed(parts).into()),
            MessagePointer::Option => Ok(Option::from_str_parts_message_assumed(parts)?.into()),
            MessagePointer::UciOk => Ok(UciOk.into()),
            MessagePointer::ReadyOk => Ok(ReadyOk.into()),
        }
    }
}
