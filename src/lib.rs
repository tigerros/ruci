#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::must_use_candidate)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(
    clippy::arithmetic_side_effects,
    clippy::unchecked_duration_subtraction,
    clippy::as_conversions,
    clippy::large_futures,
    clippy::large_stack_arrays,
    clippy::large_stack_frames,
    clippy::modulo_one,
    clippy::iterator_step_by_zero,
    clippy::invalid_regex,
    clippy::print_stdout,
    clippy::print_stderr
)]
#![deny(
    clippy::unwrap_used,
    unsafe_code,
    clippy::panic,
    clippy::exit,
    clippy::todo,
    clippy::unreachable,
    clippy::panic_in_result_fn,
    clippy::indexing_slicing,
    clippy::string_slice
)]
#![cfg_attr(not(feature = "engine-connection"), no_std)]
//! You can get started with [`Message`], but keep in mind that all messages (even those which
//! are void, like [`UciOk`]), implement [`FromStr`] and [`Display`], so you can (and should) use them
//! individually.
//!
//! # A note on "layers"
//! The [`Message`] enum wraps around the engine/GUI variants, which wraps around a specific message,
//! which is a lot of layers.
//!
//! So if you use the more general enums like the top-level [`Message`],
//! you might find yourself writing code like:
//!
//! ```ignore
//! Message::Gui(gui::Message::Go(gui::Go { .. }))
//! ```
//!
//! Know that there is an easier way! The "higher level" messages implement [`From`] for the "lower level"
//! messages, which means that [`Message`] and [`gui::Message`] implement [`From`] for [`Go`] (or any other message).
//!
//! So just do:
//!
//! ```ignore
//! gui::Go { .. }.into()
//! ```
//!
//! # A note on [`Display`] impls
//! [`Display`] implementations of messages do **not** include the final newline (`\n`) character.

extern crate alloc;
extern crate core;

mod dev_macros;
pub mod engine;
#[cfg(feature = "engine-connection")]
mod engine_connection;
pub mod errors;
pub mod gui;
mod parsing;
mod uci_moves;

use crate::engine::{BestMove, CopyProtection, Id, Info, ReadyOk, Registration, UciOk};
use crate::errors::MessageParseError;
use crate::gui::{
    Debug, Go, IsReady, PonderHit, Position, Quit, Register, SetOption, Stop, Uci, UciNewGame,
};
use core::fmt::{Display, Formatter};
use core::str::FromStr;
#[cfg(feature = "engine-connection")]
pub use engine_connection::*;
pub use uci_moves::UciMoves;

pub(crate) trait OptionReplaceIf<T> {
    fn replace_if(&mut self, other: Option<T>);
}

impl<T> OptionReplaceIf<T> for Option<T> {
    fn replace_if(&mut self, other: Self) {
        if let Some(other) = other {
            *self = Some(other);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Any message, sent from the engine or GUI.
pub enum Message {
    Engine(engine::Message),
    Gui(gui::Message),
}

impl FromStr for Message {
    type Err = MessageParseError;

    /// Tries to parse one line to a [`Message`].
    /// If there's a newline character (`\n`) present, only the first line will be processed.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (message_pointer, parts) =
            parsing::collect_any_message::<MessagePointer>("any UCI message", s)?;

        match message_pointer {
            MessagePointer::Engine(engine) => match engine {
                // Value-less, parameter-less messages
                engine::pointers::MessagePointer::UciOk => Ok(UciOk.into()),
                engine::pointers::MessagePointer::ReadyOk => Ok(ReadyOk.into()),
                // Messages with values/parameters
                engine::pointers::MessagePointer::Id => {
                    Ok(Id::from_str_parts_message_assumed(parts)?.into())
                }
                engine::pointers::MessagePointer::BestMove => {
                    Ok(BestMove::from_str_parts_message_assumed(parts).into())
                }
                engine::pointers::MessagePointer::CopyProtection => {
                    Ok(CopyProtection::from_str_parts_message_assumed(parts)?.into())
                }
                engine::pointers::MessagePointer::Registration => {
                    Ok(Registration::from_str_parts_message_assumed(parts)?.into())
                }
                engine::pointers::MessagePointer::Info => {
                    Ok(Info::from_str_parts_message_assumed(parts).into())
                }
                engine::pointers::MessagePointer::Option => {
                    Ok(engine::Option::from_str_parts_message_assumed(parts)?.into())
                }
            },
            MessagePointer::Gui(gui) => match gui {
                // Value-less, parameter-less messages
                gui::pointers::MessagePointer::Uci => Ok(Uci.into()),
                gui::pointers::MessagePointer::IsReady => Ok(IsReady.into()),
                gui::pointers::MessagePointer::UciNewGame => Ok(UciNewGame.into()),
                gui::pointers::MessagePointer::Stop => Ok(Stop.into()),
                gui::pointers::MessagePointer::PonderHit => Ok(PonderHit.into()),
                gui::pointers::MessagePointer::Quit => Ok(Quit.into()),
                // Messages with values/parameters
                gui::pointers::MessagePointer::Debug => {
                    Ok(Debug::from_str_parts_message_assumed(parts)?.into())
                }
                gui::pointers::MessagePointer::SetOption => {
                    Ok(SetOption::from_str_parts_message_assumed(parts)?.into())
                }
                gui::pointers::MessagePointer::Register => {
                    Ok(Register::from_str_parts_message_assumed(parts)?.into())
                }
                gui::pointers::MessagePointer::Position => {
                    Ok(Position::from_str_parts_message_assumed(parts).into())
                }
                gui::pointers::MessagePointer::Go => {
                    Ok(Go::from_str_parts_message_assumed(parts).into())
                }
            },
        }
    }
}

impl Display for Message {
    /// Always end with a newline.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Engine(e) => e.fmt(f),
            Self::Gui(g) => g.fmt(f),
        }
    }
}

/// This is not an actual pointer; it's just a [`Copy`] enum for referencing messages.
enum MessagePointer {
    Engine(engine::pointers::MessagePointer),
    Gui(gui::pointers::MessagePointer),
}

impl FromStr for MessagePointer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<engine::pointers::MessagePointer>().map_or_else(
            |()| s.parse::<gui::pointers::MessagePointer>().map(Self::Gui),
            |engine| Ok(Self::Engine(engine)),
        )
    }
}
