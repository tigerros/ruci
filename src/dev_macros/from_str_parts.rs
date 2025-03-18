macro_rules! from_str_parts {
    (impl $message:ident for $parts:ident -> Result<Self, MessageParseError> $body:tt) => {
        impl $message {
            /// Like the [`FromStr`](::core::str::FromStr) impl, but assumes the message is already parsed.
            /// The `parts` should be the string separated by spaces, *without the message*.
            pub(crate) fn from_str_parts_message_assumed<'s>(
                $parts: impl Iterator<Item = &'s str>,
            ) -> Result<Self, MessageParseError> {
                $body
            }
        }

        impl ::core::str::FromStr for $message {
            type Err = MessageParseError;

            fn from_str(s: &str) -> Result<Self, MessageParseError> {
                let parts = $crate::parsing::collect_message(
                    super::pointers::MessagePointer::$message.to_string(),
                    s,
                )?;
                Self::from_str_parts_message_assumed(parts)
            }
        }
    };

    (impl $message:ident for $parts:ident -> Self $body:tt) => {
        impl $message {
            /// Like the [`FromStr`](::core::str::FromStr) impl, but assumes the message is already parsed.
            /// The `parts` should be the string separated by spaces, *without the message*.
            pub(crate) fn from_str_parts_message_assumed<'s>(
                $parts: impl Iterator<Item = &'s str>,
            ) -> Self {
                $body
            }
        }

        impl ::core::str::FromStr for $message {
            type Err = $crate::errors::MessageParseError;

            /// # Errors
            /// Guaranteed to only be [`MessageParseError::NoMessage`](crate::errors::MessageParseError::NoMessage), parsing after that cannot fail.
            fn from_str(s: &str) -> Result<Self, $crate::errors::MessageParseError> {
                let parts = $crate::parsing::collect_message(
                    super::pointers::MessagePointer::$message.to_string(),
                    s,
                )?;
                Ok(Self::from_str_parts_message_assumed(parts))
            }
        }
    };
}

pub(crate) use from_str_parts;