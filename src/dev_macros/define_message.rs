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
        $(
        ::paste::paste! {
            mod [< $empty_message_ident:snake >] {
                #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                $(#[$empty_message_attr])*
                #[doc = concat!("\n\n<https://backscattering.de/chess/uci/#", stringify!([< $ident:lower >]), "-", stringify!([< $empty_message_ident:lower >]), ">")]
                pub struct $empty_message_ident;

                impl super::traits::sealed::Message for $empty_message_ident {}
                impl super::traits::Message for $empty_message_ident {}
                $crate::dev_macros::message_from_impl!([< $ident:lower >] $empty_message_ident);
                $crate::dev_macros::from_str_parts!(impl $empty_message_ident for _parts -> Self  {
                    Self
                });

                impl ::core::fmt::Display for $empty_message_ident {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str(stringify!([< $empty_message_ident:lower >]))
                    }
                }

                #[cfg(test)]
                mod tests {
                    extern crate alloc;

                    use alloc::string::ToString;
                    use super::$empty_message_ident;
                    use crate::{Message, errors::MessageParseError};
                    use pretty_assertions::{assert_eq, assert_matches};
                    use core::str::FromStr;

                    #[test]
                    fn to_from_str() {
                        let repr: Message = $empty_message_ident.into();

                        assert_eq!(repr.to_string(), stringify!([< $empty_message_ident:lower >]));
                        assert_eq!(Message::from_str(concat!(stringify!([< $empty_message_ident:lower >]), " ddd")), Ok(repr));

                        let repr: super::super::Message = $empty_message_ident.into();

                        assert_eq!(repr.to_string(), stringify!([< $empty_message_ident:lower >]));
                        assert_eq!(super::super::Message::from_str(concat!(stringify!([< $empty_message_ident:lower >]), "  \t foo\n\n")), Ok(repr));
                    }

                    #[test]
                    fn parse_error() {
                        assert_matches!($empty_message_ident::from_str("nope"), Err(MessageParseError::NoMessage { .. }))
                    }
                }
            }

            pub use [< $empty_message_ident:snake >]::*;
        }
        )*
        
        pub mod traits {
            pub(crate) mod sealed {
                pub trait Message {}
            }
            
            pub trait Message: sealed::Message {}
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

        impl traits::sealed::Message for Message<'_> {}
        impl traits::Message for Message<'_> {}
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
                #[derive(Debug, Eq, PartialEq)]
                pub enum [< $custom_message_ident ParameterPointer >] {
                    $($parameter_ident),+
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