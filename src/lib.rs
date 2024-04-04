#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
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

mod define_message_enum;
pub mod messages;
mod traits;
mod raw_uci_message;
mod uci_move_list;

pub(crate) use define_message_enum::define_message_enum;
pub use traits::*;
pub use raw_uci_message::*;
pub use uci_move_list::UciMoveList;
use messages::engine_to_gui::*;
use messages::gui_to_engine::*;

use shakmaty::uci::Uci as UciMove;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};

pub enum Message {
    GuiToEngine(GuiToEngineMessage),
    EngineToGui(EngineToGuiMessage),
}

// TODO: Finish. Take inspiration from the "uci" crate. Can't use that crate directly cause of bad, panicking code.
pub struct GuiToEngineUci {
    pub process: Child,
}

#[derive(Debug)]
pub enum UciWriteError {
    StdinIsNone,
    Io(io::Error)
}

#[derive(Debug)]
pub enum UciReadError {
    StdoutIsNone,
    Io(io::Error)
}

impl GuiToEngineUci {
    pub fn new(engine_path: &str) -> io::Result<Self> {
        let cmd = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        Ok(Self { process: cmd })
    }

    pub fn write_all(&mut self, message: &str) -> Result<(), UciWriteError> {
        let Some(stdin) = &mut self.process.stdin else {
            return Err(UciWriteError::StdinIsNone);
        };

        stdin.write_all(message.as_bytes()).map_err(UciWriteError::Io)?;

        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String, UciReadError> {
        let Some(stdout) = &mut self.process.stdout else {
            return Err(UciReadError::StdoutIsNone);
        };

        let mut reader = BufReader::with_capacity(100, stdout);

        let mut buf = String::with_capacity(100);
        reader.read_line(&mut buf).map_err(UciReadError::Io)?;

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {}
