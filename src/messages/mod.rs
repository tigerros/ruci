mod engine;
mod gui;
mod raw_gui_message;
mod raw_engine_message;
pub(crate) use raw_gui_message::RawGuiMessage;
pub(crate) use raw_engine_message::RawEngineMessage;
//pub use engine::{BestMove, CopyProtection, Id, Info, Option, Registration, option, info};
//pub use gui::{Go, Register, SetOption, SetPosition};
pub use gui::{GuiMessage, Go, Register, SetOption, SetPosition};

pub mod pointers {
    pub use super::gui::pointers as gui;
    pub use super::engine::pointers as engine;
}

pub use engine::{EngineMessage, info, Info, BestMove, CopyProtection, Id, option, Option, Registration};