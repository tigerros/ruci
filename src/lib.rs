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

mod gui_to_engine_message;
mod engine_to_gui_message;
pub use gui_to_engine_message::*;
pub use engine_to_gui_message::*;

use std::process::{Child, Command, Stdio};
use std::io;
use shakmaty::uci::Uci as UciMove;

pub(crate) fn join_uci_moves(moves: &[UciMove]) -> String {
    // AFAIK the maximum length of a UCI move is 5 chars
    let mut moves_joined = String::with_capacity(moves.len().saturating_mul(5));

    for r#move in moves {
        moves_joined.push_str(&r#move.to_string());
        moves_joined.push(' ');
    }
    
    moves_joined
}

pub enum Message {
    GuiToEngine(GuiToEngineMessage),
    EngineToGui(EngineToGuiMessage)
}

// TODO: Finish. Take inspiration from the "uci" crate. Can't use that crate directly cause of bad, panicking code.
pub struct GuiToEngineUci {
    process: Child
}

pub enum ConnectionReadLineError {
    StdoutIsNone,
    ReadIOError(io::Error)
}

pub enum UciSendMessageError {
    StdinIsNone,
}

impl GuiToEngineUci {
    pub fn new(engine_path: &str) -> io::Result<Self> {
        let cmd = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        Ok(Self {
            process: cmd
        })
    }

    pub fn send_message(&mut self, message: Message) -> Result<(), UciSendMessageError> {
        let Some(stdin) = &mut self.process.stdin else {
            return Err(UciSendMessageError::StdinIsNone);
        };

        //stdin.write_all();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
}
