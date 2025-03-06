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
    clippy::unreachable,
    clippy::unchecked_duration_subtraction,
    clippy::todo,
    clippy::string_slice,
    clippy::panic_in_result_fn,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::exit,
    clippy::as_conversions,
    clippy::large_futures,
    clippy::large_stack_arrays,
    clippy::large_stack_frames,
    clippy::modulo_one,
    clippy::mem_replace_with_uninit,
    clippy::iterator_step_by_zero,
    clippy::invalid_regex,
    clippy::print_stdout,
    clippy::print_stderr
)]

mod define_message;
pub mod engine;
#[cfg(feature = "engine-connection")]
mod engine_connection;
mod errors;
pub mod gui;
mod message_from_impl;
mod raw_message;
mod uci_moves;

use crate::engine::{BestMove, CopyProtection, Id, Info, Registration};
use crate::errors::MessageParseError;
use crate::gui::{Go, Register, SetOption, SetPosition};
use crate::raw_message::RawMessage;
#[cfg(feature = "engine-connection")]
pub use engine_connection::*;
use std::fmt::Display;
use std::str::FromStr;
pub use uci_moves::UciMoves;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    Engine(engine::Message),
    Gui(gui::Message),
}

impl FromStr for Message {
    type Err = MessageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_message = s
            .parse::<RawMessage>()
            .map_err(|_| MessageParseError::InvalidMessage)?;

        match raw_message.message_pointer {
            MessagePointer::Engine(engine) => match engine {
                // Value-less, parameter-less messages
                engine::pointers::MessagePointer::UciOk => Ok(engine::Message::UciOk.into()),
                engine::pointers::MessagePointer::ReadyOk => Ok(engine::Message::ReadyOk.into()),
                // Messages with values/parameters
                engine::pointers::MessagePointer::Id => {
                    Ok(engine::Message::Id(Id::try_from(raw_message)?).into())
                }
                engine::pointers::MessagePointer::BestMove => {
                    Ok(engine::Message::BestMove(BestMove::try_from(raw_message)?).into())
                }
                engine::pointers::MessagePointer::CopyProtection => Ok(
                    engine::Message::CopyProtection(CopyProtection::try_from(raw_message)?).into(),
                ),
                engine::pointers::MessagePointer::Registration => {
                    Ok(engine::Message::Registration(Registration::try_from(raw_message)?).into())
                }
                engine::pointers::MessagePointer::Info => {
                    Ok(engine::Message::Info(Box::new(Info::try_from(raw_message)?)).into())
                }
                engine::pointers::MessagePointer::Option => Ok(engine::Message::Option(
                    crate::engine::Option::try_from(raw_message)?,
                )
                .into()),
            },
            MessagePointer::Gui(gui) => match gui {
                // Value-less, parameter-less messages
                gui::pointers::MessagePointer::UseUci => Ok(gui::Message::UseUci.into()),
                gui::pointers::MessagePointer::IsReady => Ok(gui::Message::IsReady.into()),
                gui::pointers::MessagePointer::UciNewGame => Ok(gui::Message::UciNewGame.into()),
                gui::pointers::MessagePointer::Stop => Ok(gui::Message::Stop.into()),
                gui::pointers::MessagePointer::PonderHit => Ok(gui::Message::PonderHit.into()),
                gui::pointers::MessagePointer::Quit => Ok(gui::Message::Quit.into()),
                // Messages with values/parameters
                gui::pointers::MessagePointer::Debug => {
                    match raw_message.value.ok_or(Self::Err::MissingValue)?.as_bytes() {
                        b"on" => Ok(gui::Message::Debug(true).into()),
                        b"off" => Ok(gui::Message::Debug(false).into()),
                        _ => Err(Self::Err::ValueParseError),
                    }
                }
                gui::pointers::MessagePointer::SetOption => {
                    Ok(gui::Message::SetOption(SetOption::try_from(raw_message)?).into())
                }
                gui::pointers::MessagePointer::Register => {
                    Ok(gui::Message::Register(Register::try_from(raw_message)?).into())
                }
                gui::pointers::MessagePointer::SetPosition => {
                    Ok(gui::Message::SetPosition(SetPosition::try_from(raw_message)?).into())
                }
                gui::pointers::MessagePointer::Go => {
                    Ok(gui::Message::Go(Go::try_from(raw_message)?).into())
                }
            },
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Engine(e) => write!(f, "{e}"),
            Self::Gui(g) => write!(f, "{g}"),
        }
    }
}

impl MessagePointer {
    pub const fn has_parameters(&self) -> bool {
        match self {
            Self::Engine(p) => p.has_parameters(),
            Self::Gui(p) => p.has_parameters(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MessagePointer {
    Engine(engine::pointers::MessagePointer),
    Gui(gui::pointers::MessagePointer),
}

impl FromStr for MessagePointer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(engine) = s.parse::<engine::pointers::MessagePointer>() {
            Ok(Self::Engine(engine))
        } else {
            s.parse::<gui::pointers::MessagePointer>().map(Self::Gui)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ParameterPointer {
    Engine(engine::pointers::ParameterPointer),
    Gui(gui::pointers::ParameterPointer),
}

impl ParameterPointer {
    pub fn from_message_and_str(
        message_pointer: MessagePointer,
        s: &str,
    ) -> Result<Self, errors::ParameterPointerParseError> {
        match message_pointer {
            MessagePointer::Engine(engine_message) => {
                engine::pointers::ParameterPointer::from_message_and_str(engine_message, s)
                    .map(Self::Engine)
            }
            MessagePointer::Gui(gui_message) => {
                gui::pointers::ParameterPointer::from_message_and_str(gui_message, s).map(Self::Gui)
            }
        }
    }

    pub const fn is_void(self) -> bool {
        match self {
            Self::Engine(p) => p.is_void(),
            Self::Gui(p) => p.is_void(),
        }
    }
}
