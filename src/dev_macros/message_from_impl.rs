macro_rules! message_from_impl {
    (engine $message:ident) => {
        impl From<$message> for crate::Message {
            fn from(value: $message) -> Self {
                Self::Engine(crate::engine::Message::$message(value))
            }
        }

        impl From<$message> for crate::engine::Message {
            fn from(value: $message) -> Self {
                crate::engine::Message::$message(value)
            }
        }
    };

    (gui $message:ident) => {
        impl From<$message> for crate::Message {
            fn from(value: $message) -> Self {
                Self::Gui(crate::gui::Message::$message(value))
            }
        }

        impl From<$message> for crate::gui::Message {
            fn from(value: $message) -> Self {
                crate::gui::Message::$message(value)
            }
        }
    };
}

pub(crate) use message_from_impl;