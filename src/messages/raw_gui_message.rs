use crate::auxiliary::{MessageParameterPointer, MessagePointer, RawMessageParseError};
use crate::messages::pointers::gui::{GuiMessageParameterPointer, GuiMessagePointer};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RawGuiMessage {
    pub message_pointer: GuiMessagePointer,
    pub parameters: HashMap<GuiMessageParameterPointer, String>,
    /// Parameters like [`ponder`](https://backscattering.de/chess/uci/#gui-go-ponder) or [`infinite`](https://backscattering.de/chess/uci/#gui-go-infinite), that don't have a value.
    pub void_parameters: Vec<GuiMessageParameterPointer>,
    pub value: Option<String>,
}

impl FromStr for RawGuiMessage {
    type Err = RawMessageParseError;

    /// Should only be used with one line.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect::<Vec<_>>();

        let Some(Ok(message_pointer)) = parts.first().map(|p| GuiMessagePointer::from_str(p))
        else {
            return Err(Self::Err::NoMessage);
        };

        let Some(parts_rest) = parts.get(1..) else {
            return Ok(Self {
                message_pointer,
                parameters: HashMap::with_capacity(0),
                void_parameters: Vec::with_capacity(0),
                value: None,
            });
        };

        if !message_pointer.has_parameters() {
            return Ok(Self {
                message_pointer,
                parameters: HashMap::with_capacity(0),
                void_parameters: Vec::with_capacity(0),
                value: Some(parts_rest.join(" ")),
            });
        }

        let mut parameters = HashMap::<GuiMessageParameterPointer, String>::with_capacity(
            parts.len().saturating_div(2).saturating_sub(1),
        );
        let mut void_parameters = Vec::with_capacity(2);
        let mut value = String::with_capacity(30);
        let mut value_override = None::<String>;
        let mut last_parameter = None::<GuiMessageParameterPointer>;
        let mut first_parameter_encountered = false;

        for part in parts_rest {
            //println!("Part: {part}");
            let Ok(parameter_pointer) =
                GuiMessageParameterPointer::from_message_and_str(message_pointer, part)
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

            //println!("\tParameter pointer: {:?}", parameter_pointer);
            //println!("\tValue: [{value}]");
            //println!("\tLast parameter: {last_parameter:#?}");

            if let Some(last_parameter_some) = last_parameter {
                //println!("\tInserting last_param_some [{:?};{:?}]", last_parameter_some, value.trim());
                parameters.insert(last_parameter_some, value.trim().to_string());
                value = String::with_capacity(30);
            }

            if parameter_pointer.has_value() {
                last_parameter = Some(parameter_pointer);
                //println!("\tLast param: {:?}", last_parameter);
            } else {
                //println!("Void param: {parameter_pointer:?}");
                void_parameters.push(parameter_pointer);
                value = String::with_capacity(30);
                last_parameter = None;
            }
        }

        if let Some(last_parameter_some) = last_parameter {
            //println!("\tInserting last_param_some [{:?};{:?}]", last_parameter_some, value.trim());
            parameters.insert(last_parameter_some, value.trim().to_string());
        }

        Ok(Self {
            message_pointer,
            parameters,
            void_parameters,
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
