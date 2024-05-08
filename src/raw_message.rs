use crate::MessageParameterPointer;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageTryFromRawMessageError<MessageParameterPtr>
where
    MessageParameterPtr: MessageParameterPointer,
{
    /// This error only occurs when you are trying to parse a message directly, i.e.,
    /// trying to get [`InfoMessage`](crate::messages::InfoMessage) directly instead of [`EngineMessage`](crate::messages::EngineMessage).
    InvalidMessage,
    ParameterParseError(MessageParameterPtr),
    MissingParameter(MessageParameterPtr),
    ValueParseError,
    MissingValue,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawMessageParseError {
    NoMessage,
}
