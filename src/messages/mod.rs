mod engine;
mod gui;
mod raw_engine_message;
mod raw_gui_message;
pub use raw_engine_message::RawEngineMessage;
pub use raw_gui_message::RawGuiMessage;

// This is not redundant
#[allow(clippy::redundant_pub_crate)]
pub(crate) mod pointers {
    pub use super::engine::pointers as engine;
    pub use super::gui::pointers as gui;
}

pub use engine::{info::*, option::*, BestMove, CopyProtection, EngineMessage, Id, Registration};
pub use gui::{Go, GuiMessage, Register, SetOption, SetPosition};
