use crate::{MessageTryFromRawUciMessageError, RawUciMessage, RawUciMessageParseError};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageParseError<MessageParameterPtr>
where
    MessageParameterPtr: MessageParameterPointer,
{
    RawUciMessageParseError(RawUciMessageParseError),
    MessageTryFromRawUciMessageError(MessageTryFromRawUciMessageError<MessageParameterPtr>),
}

pub trait Message:
    Debug
    + TryFrom<RawUciMessage<Self>, Error = MessageTryFromRawUciMessageError<Self::ParameterPointer>>
    + Display
{
    type Pointer: MessagePointer;
    type ParameterPointer: MessageParameterPointer<MessagePointer = Self::Pointer>;
    fn pointer(&self) -> Self::Pointer;

    /// Tries to parse a string into this message.
    ///
    /// # Errors
    ///
    /// - Errors with [`MessageParseError::RawUciMessageParseError`] if the string could not be parsed into a [`RawUciMessage`].
    /// - Errors with [`MessageParseError::MessageTryFromRawUciMessageError`] if the [`RawUciMessage`] could not be parsed into [`Self`].
    fn from_str(s: &str) -> Result<Self, MessageParseError<Self::ParameterPointer>> {
        let raw_uci_message = RawUciMessage::<Self>::from_str(s)
            .map_err(MessageParseError::RawUciMessageParseError)?;

        Self::try_from(raw_uci_message).map_err(MessageParseError::MessageTryFromRawUciMessageError)
    }
}

pub trait MessagePointer: Copy + FromStr + Debug + Hash + Eq + PartialEq {
    fn as_string(self) -> &'static str;
    /// Whether or not this message has parameters.
    /// For example, [`uciok`](https://backscattering.de/chess/uci/#engine-uciok) does not.
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
    /// - [`MessageParameterPointerParseError::MessageHasNoParameters`]: If the `message_pointer` argument
    /// points to a message without parameters, such as [`uciok`](https://backscattering.de/chess/uci/#engine-uciok).
    /// - [`MessageParameterPointerParseError::StringDoesNotMapToParameterPointer`]: If the `message_pointer` argument
    /// points to a message which *does* have parameters, but the `s` argument doesn't match any of them.
    /// For example, if you pass in a message pointer for the [`id`](https://backscattering.de/chess/uci/#engine-id) command, but the `s` argument
    /// is `"developer"`, this will error because `id` only has `name` and `author` parameters.
    fn from_message_and_str(
        message_pointer: Self::MessagePointer,
        s: &str,
    ) -> Result<Self, MessageParameterPointerParseError>;
    /// Whether or not this parameter has a value.
    /// Almost all do, but [`ponder`](https://backscattering.de/chess/uci/#gui-go-ponder)
    /// and [`infinite`](https://backscattering.de/chess/uci/#gui-go-infinite) of the
    /// [`go`](https://backscattering.de/chess/uci/#gui-go) message don't.
    /// They are referred to as "void parameters".
    fn has_value(self) -> bool;
}
