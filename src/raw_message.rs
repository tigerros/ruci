use crate::{MessagePointer, ParameterPointer};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RawMessage {
    pub message_pointer: MessagePointer,
    pub parameters: HashMap<ParameterPointer, String>,
    /// Parameters like [`ponder`](https://backscattering.de/chess/uci/#gui-go-ponder) or [`infinite`](https://backscattering.de/chess/uci/#gui-go-infinite), that don't have a value.
    pub void_parameters: Vec<ParameterPointer>,
    /// The `var` parameter of the `option` message is the only one with multiple values.
    pub option_vars: Vec<String>,
    pub value: Option<String>,
}

impl FromStr for RawMessage {
    type Err = ();

    /// Should only be used with one line.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = if let Some((s1, _)) = s.split_once('\n') {
            s1
        } else {
            s
        };

        let parts = s.trim().split(' ').collect::<Vec<_>>();

        let Some(Ok(message_pointer)) = parts.first().map(|p| MessagePointer::from_str(p)) else {
            return Err(());
        };

        let Some(parts_rest) = parts.get(1..) else {
            return Ok(Self {
                message_pointer,
                parameters: HashMap::new(),
                void_parameters: Vec::new(),
                option_vars: Vec::new(),
                value: None,
            });
        };

        if !message_pointer.has_parameters() {
            return Ok(Self {
                message_pointer,
                parameters: HashMap::new(),
                void_parameters: Vec::new(),
                option_vars: Vec::new(),
                value: Some(parts_rest.join(" ")),
            });
        }

        let mut parameters = HashMap::<ParameterPointer, String>::with_capacity(
            parts.len().saturating_div(2).saturating_sub(1),
        );
        let mut void_parameters = Vec::with_capacity(2);
        let mut option_vars = if message_pointer == crate::engine::pointers::MessagePointer::Option.into() {
            Vec::with_capacity(10)
        } else {
            Vec::new()
        };
        let mut value = String::with_capacity(30);
        let mut value_override = None::<String>;
        let mut last_parameter = None::<ParameterPointer>;
        let mut first_parameter_encountered = false;

        for part in parts_rest {
            let Ok(parameter_pointer) =
                ParameterPointer::from_message_and_str(message_pointer, part)
            else {
                value.push_str(part);
                value.push(' ');
                continue;
            };

            if !first_parameter_encountered {
                value_override = Some(value.trim().to_string());
                value = String::with_capacity(30);
            }

            first_parameter_encountered = true;

            if let Some(last_parameter) = last_parameter {
                if last_parameter == crate::engine::pointers::OptionParameterPointer::Var.into() {
                    option_vars.push(value.trim().to_string());
                } else {
                    parameters.insert(last_parameter, value.trim().to_string());
                }

                value = String::with_capacity(30);
            }

            if parameter_pointer.is_void() {
                void_parameters.push(parameter_pointer);
                value = String::with_capacity(30);
                last_parameter = None;
            } else {
                last_parameter = Some(parameter_pointer);
            }
        }

        if let Some(last_parameter) = last_parameter {
            if last_parameter == crate::engine::pointers::OptionParameterPointer::Var.into() {
                option_vars.push(value.trim().to_string());
            } else {
                parameters.insert(last_parameter, value.trim().to_string());
            }
        }

        Ok(Self {
            message_pointer,
            parameters,
            void_parameters,
            option_vars,
            value: if value_override.is_some() {
                value_override
            } else if value.is_empty() {
                None
            } else {
                Some(value.trim().to_string())
            },
        })
    }
}
