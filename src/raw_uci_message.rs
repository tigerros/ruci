use crate::{MessageParameterPointer, MessagePointer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

/// Represents a semi-parsed UCI message.
#[derive(Debug, Clone)]
pub struct RawUciMessage<MessagePtr, MessageParameterPtr>
where
    MessagePtr: MessagePointer,
    MessageParameterPtr: MessageParameterPointer<MessagePointer = MessagePtr>,
{
    pub message_pointer: MessagePtr,
    pub parameters: HashMap<MessageParameterPtr, String>,
    /// Parameters like [`ponder`](https://backscattering.de/chess/uci/#gui-go-ponder) or [`infinite`](https://backscattering.de/chess/uci/#gui-go-infinite), that don't have a value.
    pub void_parameters: Vec<MessageParameterPtr>,
    pub value: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageTryFromRawUciMessageError<MessageParameterPtr>
where
    MessageParameterPtr: MessageParameterPointer,
{
    ParameterParseError(MessageParameterPtr),
    MissingParameter(MessageParameterPtr),
    ValueParseError,
    MissingValue,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let parts = s.trim().split(' ').collect::<Vec<_>>();

        let Some(Ok(message_pointer)) = parts.first().map(|p| MessagePtr::from_str(p)) else {
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

        let mut parameters = HashMap::<MessageParameterPtr, String>::with_capacity(
            parts.len().saturating_div(2).saturating_sub(1),
        );
        let mut void_parameters = Vec::with_capacity(2);
        let mut value = String::with_capacity(30);
        let mut last_parameter = None::<MessageParameterPtr>;

        for part in parts_rest {
            //println!("Part: {part}");
            let Ok(parameter_pointer) =
                MessageParameterPtr::from_message_and_str(message_pointer, part)
            else {
                value.push_str(part);
                value.push(' ');
                continue;
            };

            //println!("\tParameter pointer: {}", parameter_pointer.as_string());
            //println!("\tValue: [{value}]");
            //println!("\tLast parameter: {last_parameter:#?}");

            if let Some(last_parameter_some) = last_parameter {
                parameters.insert(last_parameter_some, value.trim().to_string());
                value = String::with_capacity(30);
            }

            if parameter_pointer.has_value() {
                last_parameter = Some(parameter_pointer);
            } else {
                void_parameters.push(parameter_pointer);
                value = String::with_capacity(30);
                last_parameter = None;
            }
        }

        if let Some(last_parameter) = last_parameter {
            value.pop();
            parameters.insert(last_parameter, value.trim().to_string());
        }

        Ok(Self {
            message_pointer,
            parameters,
            void_parameters,
            value: if value.is_empty() { None } else { Some(value) },
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

        for void_parameter in &self.void_parameters {
            f.write_char(' ')?;
            f.write_str(void_parameter.as_string())?;
        }

        f.write_char('\n')
    }
}
