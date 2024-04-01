use crate::{MessageParameterPointer, MessagePointer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Clone)]
/// Represents a semi-parsed UCI message.
pub struct RawUciMessage<MessagePtr, MessageParameterPtr>
where
    MessagePtr: MessagePointer,
    MessageParameterPtr: MessageParameterPointer<MessagePointer = MessagePtr>,
{
    pub message_pointer: MessagePtr,
    pub parameters: HashMap<MessageParameterPtr, String>,
    pub value: Option<String>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum RawUciMessageParseError {
    NoMessage,
}

impl<MessagePtr, MessageParameterPtr> FromStr for RawUciMessage<MessagePtr, MessageParameterPtr>
where
    MessagePtr: MessagePointer,
    MessageParameterPtr: MessageParameterPointer<MessagePointer = MessagePtr>,
{
    type Err = RawUciMessageParseError;

    /// Should only be used with one line.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();

        let Some(Ok(message_pointer)) = parts.first().map(|p| MessagePtr::from_str(p)) else {
            return Err(Self::Err::NoMessage);
        };

        let Some(parts_rest) = parts.get(1..) else {
            return Ok(Self {
                message_pointer,
                parameters: HashMap::with_capacity(0),
                value: None,
            });
        };

        if !message_pointer.has_parameters() {
            return Ok(Self {
                message_pointer,
                parameters: HashMap::with_capacity(0),
                value: Some(parts_rest.join(" ")),
            });
        }

        let mut parameters = HashMap::<MessageParameterPtr, String>::with_capacity(
            parts.len().saturating_div(2).saturating_sub(1),
        );
        let mut value = None::<String>;
        let mut current_value = String::with_capacity(30);
        let mut last_parameter = None::<MessageParameterPtr>;

        for part in parts_rest {
            //println!("Part: {part}");
            if let Ok(parameter_pointer) =
                MessageParameterPtr::from_message_and_str(message_pointer, part)
            {
                //println!("This part is a parameter");

                if value.is_none() {
                    value = Some(current_value.trim().to_string());
                    current_value = String::with_capacity(30);
                } else if let Some(last_parameter_some) = last_parameter {
                    //println!("Last parameter exists: {last_parameter_some:?}");
                    parameters.insert(last_parameter_some, current_value.trim().to_string());
                    current_value = String::with_capacity(30);
                }

                last_parameter = Some(parameter_pointer);
            } else {
                current_value.push_str(part);
                current_value.push(' ');
                //println!("Adding part to value, value: {current_value}");
            }
        }

        if let Some(last_parameter) = last_parameter {
            current_value.pop();
            //println!("After loop adding last parameter and value: {last_parameter:?}, {current_value}");
            parameters.insert(last_parameter, current_value);
        }

        //println!("{parameters:#?}");

        Ok(Self {
            message_pointer,
            parameters,
            value,
        })
    }
}

impl<MessagePtr, MessageParameterPtr> Display for RawUciMessage<MessagePtr, MessageParameterPtr>
where
    MessagePtr: MessagePointer,
    MessageParameterPtr: MessageParameterPointer<MessagePointer = MessagePtr>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message_pointer.as_string())?;

        for (parameter, parameter_value) in &self.parameters {
            f.write_char(' ')?;
            f.write_str(parameter.as_string())?;
            f.write_char(' ')?;
            f.write_str(parameter_value)?;
        }

        f.write_char('\n')
    }
}