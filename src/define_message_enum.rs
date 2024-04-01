macro_rules! define_message_enum {
    (empty_string=$t:tt) => {""};

    (
        $(#[$attr:meta])*
        $vis:vis enum $ident:ident {
            $(
            $(#[$message_attr:meta])*
            %[$message_string:literal]
            $(%$has_parameters:tt[parameters = [$($(*$has_value:tt)?($message_parameter_ident:ident, $message_parameter_string:literal)),+]])?
            $message_ident:ident
            $(($($message_argument:ty),+))?
            $({ $($message_field:ident: $message_field_ty:ty),+ })?
            ),+
        }
    ) => {
        $(#[$attr])*
        $vis enum $ident {
            $(
            $(#[$message_attr])*
            $message_ident
            $(($($message_argument),+))?
            $({ $($message_field: $message_field_ty),+ })?
            ),+
        }

        ::paste::paste! {
            impl $ident {
                pub fn pointer(&self) -> [< $ident Pointer >] {
                    match self {
                        $(
                        Self::$message_ident $(($($message_argument),+))? $({ $($message_field: $message_field_ty),+ })? => [< $ident Pointer >]::$message_ident
                        ),+
                    }
                }
            }
            
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
            $vis enum [< $ident Pointer >] {
                $(
                $(#[$message_attr])*
                $message_ident
                ),+
            }

            #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
            $vis enum [< $ident ParameterPointer >] {
				$($(
                #[allow(unused_doc_comments)]
                #[doc = define_message_enum!(empty_string=$has_parameters)]
				[< $message_ident:camel >]( [< $ident $message_ident:camel ParameterPointer >]),
				)?)+
			}

			$($(
			#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
            $vis enum [< $ident $message_ident ParameterPointer >] {
				$(
				$message_parameter_ident
				),+
			}
			)?)+

            impl ::std::str::FromStr for [< $ident Pointer >] {
                type Err = ();

                /// Parses a string to a message pointer.
                ///
                /// The error type is blank because there is only one error case: the string simply didn't match a message.
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $($message_string => Ok(Self::$message_ident),)+
                        _ => Err(())
                    }
                }
            }

            impl $crate::MessageParameterPointer for [< $ident ParameterPointer >] {
                type MessagePointer = [< $ident Pointer >];

                fn as_string(self) -> &'static str {
                    match self {
                        $($(
                        #[allow(unused_doc_comments)]
                        #[doc = define_message_enum!(empty_string=$has_parameters)]
                        Self::$message_ident(parameter_pointer) => match parameter_pointer {
                            $(
                            [< $ident $message_ident ParameterPointer >]::$message_parameter_ident => $message_parameter_string,
                            )+
                        },
                        )?)+
                    }
                }

                fn from_message_and_str(message_pointer: Self::MessagePointer, s: &str) -> Result<Self, $crate::MessageParameterPointerParseError> {
                    match message_pointer {
                        $($(
                        #[allow(unused_doc_comments)]
                        #[doc = define_message_enum!(empty_string=$has_parameters)]
                        Self::MessagePointer::$message_ident => match s {
                            $(
                            $message_parameter_string => Ok(Self::[< $message_ident >]([< $ident $message_ident ParameterPointer >]::$message_parameter_ident)),
                            )+
                            _ => Err($crate::MessageParameterPointerParseError::StringDoesNotMapToParameterPointer)
                        },
                        )?)+
                        _ => Err($crate::MessageParameterPointerParseError::MessageHasNoParameters)
                    }
                }

                #[allow(unreachable_patterns)]
                fn has_value(self) -> bool {
                    match self {
                        $($(
                        #[allow(unused_doc_comments)]
                        #[doc = define_message_enum!(empty_string=$has_parameters)]
                        Self::$message_ident(parameter_pointer) => match parameter_pointer {
                            $($(
                            #[allow(unused_doc_comments)]
                            #[doc = define_message_enum!(empty_string=$has_value)]
                            [< $ident $message_ident ParameterPointer >]::$message_parameter_ident => true,
                            )?)+
                            _ => false
                        },
                        )?)+
                        _ => false
                    }
                }
            }

            impl $crate::MessagePointer for [< $ident Pointer >] {
                fn as_string(self) -> &'static str {
                    match self {
                        $(Self::$message_ident => $message_string),+
                    }
                }

                fn has_parameters(self) -> bool {
                    match self {
                        $($(
                        #[allow(unused_doc_comments)]
                        #[doc = define_message_enum!(empty_string=$has_parameters)]
                        Self::$message_ident => true,
                        )?)+
                        _ => false
                    }
                }
            }
        }
    };
}

pub(crate) use define_message_enum;
