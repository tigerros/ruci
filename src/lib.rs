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
    clippy::panic,
    clippy::exit,
    clippy::todo,
    clippy::unreachable,
    clippy::panic_in_result_fn,
    clippy::indexing_slicing,
    clippy::string_slice
)]
#![cfg_attr(not(feature = "io"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
//! # A note on "GUI"
//! Whenever you see "GUI" in the documentation, it doesn't mean "Graphical User Interface".
//! It means "whatever controls the engine".
//! It's called that because that's how it's referred to in the UCI protocol and elsewhere.
//!
//! # A note on message enums
//! If you write something like:
//! ```rust
//! # use ruci::{engine, Info, Message};
//! # use ruci::gui;
//! # use ruci::Go;
//! let _: engine::Message = engine::Message::Info(Box::new(Info::default()));
//! let _: Message = Message::Gui(gui::Message::Go(Go::default()));
//! ```
//!
//! Use instead:
//! ```rust
//! # use ruci::{engine, Info, Message};
//! # use ruci::gui;
//! # use ruci::Go;
//! let _: engine::Message = Info::default().into();
//! let _: Message = Go::default().into();
//! ```
//!
//! In short, lower level messages can be converted to the higher level.
//!
//! # A note on [`Display`] impls
//! [`Display`] implementations of messages do **not** include the final newline (`\n`) character.

extern crate alloc;

mod dev_macros;
pub mod engine;
#[cfg(feature = "engine-sync")]
mod engine_connection;
#[cfg(feature = "engine-async")]
mod engine_connection_async;
mod errors;
pub mod gui;
#[cfg(feature = "gui-sync")]
mod gui_connection;
#[cfg(feature = "gui-async")]
mod gui_connection_async;
mod parsing;
mod uci_moves;

use core::fmt::{Display, Formatter};
use core::str::FromStr;
#[cfg(feature = "engine-sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "engine-sync")))]
pub use engine_connection::*;
pub use errors::*;
#[cfg(feature = "gui-sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "gui-sync")))]
pub use gui_connection::*;
pub use {engine::*, gui::*};

pub(crate) trait OptionReplaceIf<T> {
    fn replace_if(&mut self, other: core::option::Option<T>);
}

impl<T> OptionReplaceIf<T> for core::option::Option<T> {
    fn replace_if(&mut self, other: Self) {
        if let Some(other) = other {
            *self = Some(other);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Any message, sent from the engine or GUI.
pub enum Message<'a> {
    Engine(engine::Message<'a>),
    Gui(gui::Message<'a>),
}

pub mod traits {
    use core::{
        fmt::{Debug, Display},
        hash::Hash,
    };

    pub(crate) mod sealed {
        pub trait Message {}
    }

    /// Marks all message types and their references.
    /// Intentionally sealed.
    pub trait Message: sealed::Message + Display + Debug + PartialEq + Eq + Hash + Clone {}
}

impl traits::sealed::Message for Message<'_> {}
impl traits::Message for Message<'_> {}
impl traits::sealed::Message for &'_ Message<'_> {}
impl traits::Message for &'_ Message<'_> {}

impl FromStr for Message<'_> {
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
                    Ok(Option::from_str_parts_message_assumed(parts)?.into())
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
                    Ok(Position::from_str_parts_message_assumed(parts)?.into())
                }
                gui::pointers::MessagePointer::Go => {
                    Ok(Go::from_str_parts_message_assumed(parts).into())
                }
            },
        }
    }
}

impl Display for Message<'_> {
    /// Always end with a newline.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Engine(e) => e.fmt(f),
            Self::Gui(g) => g.fmt(f),
        }
    }
}

/// This is not an actual pointer; it is just a [`Copy`] enum for referencing messages.
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
