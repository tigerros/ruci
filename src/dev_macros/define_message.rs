macro_rules! define_message {
    (
        $(#[$attr:meta])*
        enum $ident:ident {
            =[custom]
            $(
            $(%[parameters $has_parameters:tt [$($parameter_ident:ident),+]])?
            $custom_message_ident:ident
            ($custom_message_value:ty)
            ),*
            =[empty]
            $(
            $(#[$empty_message_attr:meta])*
            $empty_message_ident:ident
            ),*
        }
    ) => {
        $(::paste::paste! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            $(#[$empty_message_attr])*
            #[doc = concat!("\n\n<https://backscattering.de/chess/uci/#", stringify!([< $ident:lower >]), "-", stringify!([< $empty_message_ident:lower >]), ">")]
            pub struct $empty_message_ident;
            
            $crate::dev_macros::impl_message!($empty_message_ident);
            $crate::dev_macros::message_from_impl!([< $ident:lower >] $empty_message_ident);
            $crate::dev_macros::from_str_parts!(impl $empty_message_ident for _parts -> Self  {
                Self
            });
            
            impl ::core::fmt::Display for $empty_message_ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_str(stringify!([< $empty_message_ident:lower >]))
                }
            }
        })*
        
        #[cfg(test)]
        mod tests {$(::paste::paste! {
            mod [< $empty_message_ident:snake >] {
                extern crate alloc;
                
                use crate::errors::{MessageParseError, MessageParseErrorKind};
                use super::super::$empty_message_ident;
                use pretty_assertions::{assert_eq, assert_matches};
                use core::str::FromStr;
                use alloc::string::ToString;

                #[test]
                fn to_from_str() {
                    $crate::dev_macros::assert_message_to_from_str!([< $ident:lower >] $empty_message_ident, stringify!([< $empty_message_ident:lower >]));
                    $crate::dev_macros::assert_from_str_message!([< $ident:lower >] concat!(stringify!([< $empty_message_ident:lower >]), " ddd"), Ok($empty_message_ident));
                    $crate::dev_macros::assert_from_str_message!([< $ident:lower >] concat!("\tggg ", stringify!([< $empty_message_ident:lower >]), " \t f\n\n"), Ok($empty_message_ident));
                }
                
                #[test]
                fn parse_error() {
                    assert_eq!($empty_message_ident::from_str("nope"), Err(MessageParseError {
                        expected: stringify!([< $empty_message_ident:lower >]),
                        kind: MessageParseErrorKind::NoMessage
                    }));
                    assert_matches!(super::super::Message::from_str("\n\nnuh uh"), Err(MessageParseError {
                        expected: _,
                        kind: MessageParseErrorKind::NoMessage
                    }));
                    assert_eq!(crate::Message::from_str("𓉌𓊦   𓐧 "), Err(MessageParseError {
                        expected: "any UCI message",
                        kind: MessageParseErrorKind::NoMessage
                    }));
                }
            }
        })*}
        
        pub mod traits {
            pub(crate) mod sealed {
                pub trait Message {}
            }

            /// Marks [`super`] message types and their references.
            /// Intentionally sealed.
            pub trait Message:
                sealed::Message
                + ::core::fmt::Display
                + ::core::fmt::Debug
                + ::core::cmp::PartialEq
                + ::core::cmp::Eq
                + ::core::hash::Hash
                + ::core::clone::Clone
            {}
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        $(#[$attr])*
        pub enum Message<'a> {
            $(
            $custom_message_ident
            ($custom_message_value),
            )*
            $(
            $empty_message_ident
            ($empty_message_ident),
            )*
        }

        $crate::dev_macros::impl_message!(Message<'_>);
        impl<'a> ::core::convert::From<Message<'a>> for $crate::Message<'a> {
            fn from(value: Message<'a>) -> Self {
                Self::$ident(value)
            }
        }

        impl ::core::fmt::Display for Message<'_> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(Self::$custom_message_ident(m) => m.fmt(f),)*
                    $(Self::$empty_message_ident(m) => m.fmt(f),)*
                }
            }
        }

        pub(crate) mod pointers {
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub enum MessagePointer {
                $($custom_message_ident,)*
                $($empty_message_ident,)*
            }

            impl MessagePointer {
                pub const fn to_string(self) -> &'static str {
                    match self {
                        $(Self::$custom_message_ident => ::paste::paste!(stringify!([< $custom_message_ident:lower >])),)*
                        $(Self::$empty_message_ident => ::paste::paste!(stringify!([< $empty_message_ident:lower >])),)*
                    }
                }
            }

            impl ::core::str::FromStr for MessagePointer {
                type Err = ();

                fn from_str(s: &str) -> Result<Self, ()> {
                    ::paste::paste! { match s {
                        $(stringify!([< $custom_message_ident:lower >]) => Ok(Self::$custom_message_ident),)*
                        $(stringify!([< $empty_message_ident:lower >]) => Ok(Self::$empty_message_ident),)*
                        _ => Err(())
                    } }
                }
            }

            $($(::paste::paste! {
                #[derive(Debug, Eq, PartialEq, Copy, Clone)]
                pub enum [< $custom_message_ident ParameterPointer >] {
                    $($parameter_ident),+
                }

                impl [< $custom_message_ident ParameterPointer >] {
                    #[allow(dead_code)]
                    /// Only used for the info message.
                    /// This is the string representation of the parameter, surrounded by spaces.
                    pub const fn as_str_spaced(self) -> &'static str {
                        match self {
                            $(Self::$parameter_ident => concat!(" ", stringify!([< $parameter_ident:lower >]), " ")),+
                        }
                    }
                }

                impl ::core::str::FromStr for [< $custom_message_ident ParameterPointer >] {
                    type Err = ();

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        match s {
                            $(stringify!([< $parameter_ident:lower >]) => Ok(Self::$parameter_ident),)+
                            _ => Err(())
                        }
                    }
                }
            })?)+
        }
    };
}

pub(crate) use define_message;