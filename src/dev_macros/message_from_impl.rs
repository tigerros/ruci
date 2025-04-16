macro_rules! message_from_impl {
    (engine $message:ident $(<$l:lifetime>)?) => {
        impl<'a> From<$message $(<$l>)?> for crate::Message<'a> {
            fn from(value: $message $(<$l>)?) -> Self {
                Self::Engine(crate::engine::Message::$message(value))
            }
        }

        impl<'a> From<$message $(<$l>)?> for crate::engine::Message<'a> {
            fn from(value: $message $(<$l>)?) -> Self {
                crate::engine::Message::$message(value)
            }
        }
    };

    (gui $message:ident $(<$l:lifetime>)?) => {
        impl<'a> From<$message $(<$l>)?> for crate::Message<'a> {
            fn from(value: $message $(<$l>)?) -> Self {
                Self::Gui(crate::gui::Message::$message(value))
            }
        }

        impl<'a> From<$message $(<$l>)?> for crate::gui::Message<'a> {
            fn from(value: $message $(<$l>)?) -> Self {
                crate::gui::Message::$message(value)
            }
        }
    };
}

pub(crate) use message_from_impl;