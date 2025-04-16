macro_rules! from_str_parts {
    (@from_str_docs=Result) => {
        ""
    };
    (@from_str_docs=Self) => {
        "# Errors\nGuaranteed to only be [`MessageParseError::NoMessage`](crate::MessageParseError::NoMessage), parsing after that cannot fail."
    };
    (@message_assumed_ret=Result) => {
        Result<Self, $crate::MessageParseError>
    };
    (@message_assumed_ret=Self) => {
        Self
    };
    (@from_str_ret=Result($parts:ident)) => {
        Self::from_str_parts_message_assumed($parts)
    };
    (@from_str_ret=Self($parts:ident)) => {
        Ok(Self::from_str_parts_message_assumed($parts))
    };

    (impl $message:ident for $parts:ident -> $ret:ident $body:tt) => {
        impl $message {
            /// Like the [`FromStr`](::core::str::FromStr) impl, but assumes the message is already parsed.
            /// The `parts` should be the string separated by spaces, *without the message*.
            pub(crate) fn from_str_parts_message_assumed<'s>(
                $parts: impl Iterator<Item = &'s str>,
            ) -> $crate::dev_macros::from_str_parts!(@message_assumed_ret=$ret) {
                $body
            }
        }

        impl ::core::str::FromStr for $message {
            type Err = $crate::MessageParseError;

            #[allow(clippy::empty_docs)]
            #[doc = $crate::dev_macros::from_str_parts!(@from_str_docs=$ret)]
            fn from_str(s: &str) -> Result<Self, $crate::MessageParseError> {
                let parts = $crate::parsing::collect_message(
                    super::pointers::MessagePointer::$message.to_string(),
                    s,
                )?;
                $crate::dev_macros::from_str_parts!(@from_str_ret=$ret(parts))
            }
        }
    };

    (impl $message:ident <$l:lifetime> for $parts:ident -> $ret:ident $body:tt) => {
        impl<$l> $message<$l> {
            /// Like the [`FromStr`](::core::str::FromStr) impl, but assumes the message is already parsed.
            /// The `parts` should be the string separated by spaces, *without the message*.
            pub(crate) fn from_str_parts_message_assumed<'s>(
                $parts: impl Iterator<Item = &'s str>,
            ) -> $crate::dev_macros::from_str_parts!(@message_assumed_ret=$ret) {
                $body
            }
        }

        impl<$l> ::core::str::FromStr for $message<$l> {
            type Err = $crate::MessageParseError;

            #[allow(clippy::empty_docs)]
            #[doc = $crate::dev_macros::from_str_parts!(@from_str_docs=$ret)]
            fn from_str(s: &str) -> Result<Self, $crate::MessageParseError> {
                let parts = $crate::parsing::collect_message(
                    super::pointers::MessagePointer::$message.to_string(),
                    s,
                )?;
                $crate::dev_macros::from_str_parts!(@from_str_ret=$ret(parts))
            }
        }
    };
}

pub(crate) use from_str_parts;