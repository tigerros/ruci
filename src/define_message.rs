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

        pub mod pointers {
            /// This enum is used in parsing and is returned in errors to determine what message
            /// is being processed, or which one was the source of an error.
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
            pub enum MessagePointer {
                $($message_ident),+
            }

            impl MessagePointer {
                pub const fn to_string(self) -> &'static str {
                    match self {
                        $(Self::$message_ident => $message_string),+
                    }
                }

                pub const fn has_parameters(self) -> bool {
                    match self {
                        $($(
                        Self::$message_ident => {
                            $crate::define_message::define_message!(empty=$has_parameters);
                            true
                        },
                        )?)+
                        _ => false
                    }
                }
            }

            impl ::std::str::FromStr for MessagePointer {
                type Err = ();

                fn from_str(s: &str) -> Result<Self, ()> {
                    match s {
                        $($message_string => Ok(Self::$message_ident),)+
                        _ => Err(())
                    }
                }
            }

            impl From<MessagePointer> for crate::MessagePointer {
                fn from(value: MessagePointer) -> Self {
                    Self::$ident(value)
                }
            }

            /// Similar to [`MessagePointer`], but points to individual parameters.
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
            pub enum ParameterPointer {
                $($(
                    #[allow(unused_doc_comments)]
                    #[allow(clippy::empty_docs)]
                    #[doc = $crate::define_message::define_message!(empty_string=$has_parameters)]
                    $message_ident(::paste::paste!([< $message_ident ParameterPointer >])),
                )?)+
            }

            impl From<ParameterPointer> for crate::ParameterPointer {
                fn from(value: ParameterPointer) -> Self {
                    Self::$ident(value)
                }
            }

            ::paste::paste! {
                impl ParameterPointer {
                    pub fn from_message_and_str(message_pointer: MessagePointer, s: &str) -> Result<Self, $crate::errors::ParameterPointerParseError> {
                        match message_pointer {
                            $($(
                            MessagePointer::$message_ident => {
                                $crate::define_message::define_message!(empty=$has_parameters);
                                match s {
                                    $(
                                    $parameter_string => Ok(Self::[< $message_ident >]([< $message_ident ParameterPointer >]::$parameter_ident)),
                                    )+
                                    _ => Err($crate::errors::ParameterPointerParseError::StringDoesNotMapToParameterPointer)
                                }
                            },
                            )?)+
                            _ => Err($crate::errors::ParameterPointerParseError::MessageHasNoParameters)
                        }
                    }

                    pub const fn to_string(self) -> &'static str {
                        match self {
                            $($(
                            Self::$message_ident(parameter_pointer) => {
                                $crate::define_message::define_message!(empty=$has_parameters);
                                parameter_pointer.to_string()
                            }
                            )?),+
                        }
                    }

                    #[allow(unreachable_patterns)]
                    pub const fn is_void(self) -> bool {
                        match self {
                            $($(
                            Self::$message_ident(parameter_pointer) => {
                                $crate::define_message::define_message!(empty=$has_parameters);
                                parameter_pointer.is_void()
                            }
                            )?),+
                        }
                    }
                }
            }

            $($(::paste::paste! {
                #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
                pub enum [< $message_ident ParameterPointer >] {
                    $($parameter_ident),+
                }

                impl [< $message_ident ParameterPointer >] {
                    pub const fn to_string(self) -> &'static str {
                        match self {
                            $(Self::$parameter_ident => $parameter_string),+
                        }
                    }

                    pub const fn is_void(self) -> bool {
                        match self {
                            $($(
                            Self::$parameter_ident => {
                                $crate::define_message::define_message!(empty=$parameter_is_void);
                                true
                            },
                            )?)+
                            _ => false
                        }
                    }
                }

                impl From<[< $message_ident ParameterPointer >]> for ParameterPointer {
                    fn from(value: [< $message_ident ParameterPointer >]) -> Self {
                        Self::$message_ident(value)
                    }
                }

                impl From<[< $message_ident ParameterPointer >]> for crate::ParameterPointer {
                    fn from(value: [< $message_ident ParameterPointer >]) -> Self {
                        Self::$ident(value.into())
                    }
                }
            })?)+
        }
    };
}

pub(crate) use define_message;
