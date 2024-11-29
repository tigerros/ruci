use crate::auxiliary::{MessageParameterPointer, MessagePointer, RawMessageParseError};
use crate::messages::pointers::engine::{
    EngineMessageOptionParameterPointer, EngineMessageParameterPointer, EngineMessagePointer,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RawEngineMessage {
    pub message_pointer: EngineMessagePointer,
    pub parameters: HashMap<EngineMessageParameterPointer, String>,
    pub option_vars: Vec<String>,
    pub value: Option<String>,
}

impl FromStr for RawEngineMessage {
    type Err = RawMessageParseError;

    /// Should only be used with one line.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect::<Vec<_>>();

        let Some(Ok(message_pointer)) = parts.first().map(|p| EngineMessagePointer::from_str(p))
        else {
            return Err(Self::Err::NoMessage);
        };

        let Some(parts_rest) = parts.get(1..) else {
            return Ok(Self {
                message_pointer,
                parameters: HashMap::with_capacity(0),
                option_vars: Vec::with_capacity(0),
                value: None,
            });
        };

        if !message_pointer.has_parameters() {
            return Ok(Self {
                message_pointer,
                parameters: HashMap::with_capacity(0),
                option_vars: Vec::with_capacity(0),
                value: Some(parts_rest.join(" ")),
            });
        }

        let mut parameters = HashMap::<EngineMessageParameterPointer, String>::with_capacity(
            parts.len().saturating_div(2).saturating_sub(1),
        );
        let mut option_vars = if message_pointer == EngineMessagePointer::Option {
            Vec::with_capacity(10)
        } else {
            Vec::with_capacity(0)
        };
        let mut value = String::with_capacity(30);
        let mut value_override = None::<String>;
        let mut last_parameter = None::<EngineMessageParameterPointer>;
        let mut first_parameter_encountered = false;

        for part in parts_rest {
            //println!("Part: {part}");
            let Ok(parameter_pointer) =
                EngineMessageParameterPointer::from_message_and_str(message_pointer, part)
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
                if last_parameter_some
                    == EngineMessageParameterPointer::Option(
                        EngineMessageOptionParameterPointer::Var,
                    )
                {
                    option_vars.push(value.trim().to_string());
                } else {
                    //println!(
                    //    "\tInserting last_param_some [{:?};{:?}]",
                    //    last_parameter_some,
                    //    value.trim()
                    //);
                    parameters.insert(last_parameter_some, value.trim().to_string());
                }

                value = String::with_capacity(30);
            }

            last_parameter = Some(parameter_pointer);
            //println!("\tLast param: {:?}", last_parameter);
        }

        if let Some(last_parameter_some) = last_parameter {
            if last_parameter_some
                == EngineMessageParameterPointer::Option(EngineMessageOptionParameterPointer::Var)
            {
                option_vars.push(value.trim().to_string());
            } else {
                parameters.insert(last_parameter_some, value.trim().to_string());
            }
        }

        Ok(Self {
            message_pointer,
            parameters,
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
