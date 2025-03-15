macro_rules! define_message {
    (empty_string=$t:tt) => {""};
    (empty=$t:tt) => {};

    (
        $(#[$attr:meta])*
        enum $ident:ident {
            $(
            $(#[$message_attr:meta])*
            %[$message_string:literal]
            $(%[parameters $has_parameters:tt [$($(*$parameter_is_void:tt)?$parameter_ident:ident: $parameter_string:literal),+]])?
            $message_ident:ident
            $(($has_arguments:tt $($message_argument:ty),+))?
            ),+
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Message {
            $(
            $(#[$message_attr])*
            $message_ident
            $(($($message_argument),+))?
            ),+
        }

        impl From<Message> for crate::Message {
            fn from(value: Message) -> Self {
                Self::$ident(value)
            }
        }

        pub(crate) mod pointers {
            /// See [`crate::MessagePointer`].
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub enum MessagePointer {
                $($message_ident),+
            }

            impl MessagePointer {
                pub const fn to_string(self) -> &'static str {
                    match self {
                        $(Self::$message_ident => $message_string),+
                    }
                }
            }

            impl ::core::str::FromStr for MessagePointer {
                type Err = ();

                fn from_str(s: &str) -> Result<Self, ()> {
                    match s {
                        $($message_string => Ok(Self::$message_ident),)+
                        _ => Err(())
                    }
                }
            }

            $($(::paste::paste! {
                #[derive(Debug, Eq, PartialEq)]
                pub enum [< $message_ident ParameterPointer >] {
                    $($parameter_ident),+
                }

                impl ::core::str::FromStr for [< $message_ident ParameterPointer >] {
                    type Err = ();
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        match s {
                            $($parameter_string => Ok(Self::$parameter_ident),)+
                            _ => Err(())
                        }
                    }
                }
            })?)+
        }
    };
}

pub(crate) use define_message;
