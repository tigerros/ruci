mod engine;
mod gui;
mod raw_engine_message;
mod raw_gui_message;
pub(crate) use raw_engine_message::RawEngineMessage;
pub(crate) use raw_gui_message::RawGuiMessage;
//pub use engine::{BestMove, CopyProtection, Id, Info, Option, Registration, option, info};
//pub use gui::{Go, Register, SetOption, SetPosition};
pub use gui::{Go, GuiMessage, Register, SetOption, SetPosition};

pub mod pointers {
    pub use super::engine::pointers as engine;
    pub use super::gui::pointers as gui;
}

pub use engine::{
    info, option, BestMove, CopyProtection, EngineMessage, Id, Info, Option, Registration,
};
