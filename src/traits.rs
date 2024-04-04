use crate::RawUciMessage;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

pub trait Message:
    Debug + TryFrom<RawUciMessage<Self::MessagePointer, Self::MessageParameterPointer>> + Display
{
    type MessagePointer: MessagePointer;
    type MessageParameterPointer: MessageParameterPointer<MessagePointer = Self::MessagePointer>;
    fn pointer(&self) -> Self::MessagePointer;
}

pub trait MessagePointer: Copy + FromStr + Debug + Hash + Eq + PartialEq {
    fn as_string(self) -> &'static str;
    fn has_parameters(self) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub enum MessageParameterPointerParseError {
    MessageHasNoParameters,
    StringDoesNotMapToParameterPointer,
}

pub trait MessageParameterPointer: Copy + Debug + Hash + Eq + PartialEq {
    type MessagePointer: MessagePointer;
    fn as_string(self) -> &'static str;
    /// Parses a string to a message parameter pointer.
    ///
    /// # Errors
    ///
    /// - `MessageHasNoParameters`: errors if the `message_pointer` argument
    /// points to a message without parameters, such as `UciOk`.
    /// - `StringDoesNotMapToParameterPointer`: errors if the `message_pointer` argument
    /// points to a message which *does* have parameters, but the `s` argument doesn't match any of them.
    /// For example, if you pass in a message pointer for the `Id` command, but the `s` argument
    /// is `"developer"`, this will error because `Id` only has `name` and `author` parameters.
    fn from_message_and_str(
        message_pointer: Self::MessagePointer,
        s: &str,
    ) -> Result<Self, MessageParameterPointerParseError>;
    fn has_value(self) -> bool;
}
