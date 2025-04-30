/// Asserts that this message converted to a string equals the given string.
///
/// # Syntax
/// `gui/engine message, string`
///
/// For example,
/// `gui UciOk {}, "uciok"` will test that `UciOk, Message::from(UciOk), gui::Message::from(UciOk)`
/// converted to a string are all `"uciok"`.
macro_rules! assert_message_to_str {
    // __ here so I always specify either gui or engine
    (__ $m:expr, $s:expr) => {{
        pretty_assertions::assert_eq!($crate::Message::from($m.clone()).to_string(), $s);
        pretty_assertions::assert_eq!($m.to_string(), $s);
    }};

    (gui $m:expr, $s:expr) => {{
        $crate::dev_macros::assert_message_to_str!(__ $m, $s);
        pretty_assertions::assert_eq!($crate::gui::Message::from($m).to_string(), $s);
    }};

    (engine $m:expr, $s:expr) => {{
        $crate::dev_macros::assert_message_to_str!(__ $m, $s);
        pretty_assertions::assert_eq!($crate::engine::Message::from($m).to_string(), $s);
    }};
}

pub(crate) use assert_message_to_str;

/// Asserts that this string converted to a message equals the given message.
///
/// # Syntax
/// `gui/engine string, message`
///
/// For example,
/// `gui "uciok", Ok(UciOk {})` will test that `"uciok"` converted to
/// `UciOk, Message::from(UciOk), gui::Message::from(UciOk)` are all `UciOk {}`.
macro_rules! assert_from_str_message {
    (__ $s:expr, $m:expr) => {{
        use ::core::str::FromStr;
        pretty_assertions::assert_eq!($crate::Message::from_str($s), $m.map(|ok| $crate::Message::from(ok)));
        pretty_assertions::assert_eq!(FromStr::from_str($s), $m);
    }};

    (gui $s:expr, $m:expr) => {{
        use ::core::str::FromStr;
        $crate::dev_macros::assert_from_str_message!(__ $s, $m);
        pretty_assertions::assert_eq!($crate::gui::Message::from_str($s), $m.map(|ok| $crate::gui::Message::from(ok)));
    }};

    (engine $s:expr, $m:expr) => {{
        use ::core::str::FromStr;
        $crate::dev_macros::assert_from_str_message!(__ $s, $m);
        pretty_assertions::assert_eq!($crate::engine::Message::from_str($s), $m.map(|ok| $crate::engine::Message::from(ok)));
    }};
}

pub(crate) use assert_from_str_message;

/// Combination of [`assert_message_to_str`] and [`assert_from_str_message`],
/// using [`assert_message_to_str`] syntax (message first).
///
/// Use if the string you are converting to and from are the same.
macro_rules! assert_message_to_from_str {
    (gui $m:expr, $s:expr) => {{
        $crate::dev_macros::assert_from_str_message!(gui $s, Ok($m.clone()));
        $crate::dev_macros::assert_message_to_str!(gui $m, $s);
    }};

    (engine $m:expr, $s:expr) => {{
        $crate::dev_macros::assert_from_str_message!(engine $s, Ok($m.clone()));
        $crate::dev_macros::assert_message_to_str!(engine $m, $s);
    }};
}

pub(crate) use assert_message_to_from_str;
