use shakmaty::uci::UciMove;
use std::fmt::{Debug, Display, Formatter, Write};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A simple [`Vec<UciMove>`] wrapper that provides [`FromStr`] and [`Display`] implementations.
pub struct UciMoveList(pub Vec<UciMove>);

impl FromStr for UciMoveList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(' ').map_while(|part| part.parse().ok()).collect(),
        ))
    }
}

impl Display for UciMoveList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first_iter = true;

        for r#move in &self.0 {
            // Do not write the space on the first iteration
            if first_iter {
                first_iter = false;
            } else {
                f.write_char(' ')?;
            }

            f.write_str(&r#move.to_string())?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageParseError<MessageParameterPtr>
where
    MessageParameterPtr: MessageParameterPointer,
{
    /// There is no message in the string.
    RawMessageParseError(RawMessageParseError),
    /// The raw message could not be parsed into a concrete message.
    MessageTryFromRawMessageError(MessageTryFromRawMessageError<MessageParameterPtr>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageTryFromRawMessageError<MessageParameterPtr>
where
    MessageParameterPtr: MessageParameterPointer,
{
    /// This error only occurs when you are trying to parse a message directly, i.e.,
    /// trying to get [`InfoMessage`](crate::messages::Info) directly instead of [`EngineMessage`](crate::messages::EngineMessage).
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

pub trait Message:
    Debug + Display + FromStr<Err = MessageParseError<Self::ParameterPointer>> + Send + Sync + 'static
{
    type Pointer: MessagePointer;
    type ParameterPointer: MessageParameterPointer<MessagePointer = Self::Pointer>;
}

/// This is a simple [`Copy`] "pointer" enum necessary for parsing.
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

/// This is a simple [`Copy`] "pointer" enum necessary for parsing.
pub trait MessageParameterPointer: Copy + Debug + Hash + Eq + PartialEq {
    type MessagePointer: MessagePointer;
    fn as_string(self) -> &'static str;
    /// Parses a string to a message parameter pointer.
    ///
    /// # Errors
    ///
    /// - [`MessageParameterPointerParseError::MessageHasNoParameters`]: if the `message_pointer` argument
    ///     points to a message without parameters, such as [`uciok`](https://backscattering.de/chess/uci/#engine-uciok).
    /// - [`MessageParameterPointerParseError::StringDoesNotMapToParameterPointer`]: if the `message_pointer` argument
    ///     points to a message, which *does* have parameters, but the `s` argument doesn't match any of them.
    ///     For example, if you pass in a message pointer for the [`id`](https://backscattering.de/chess/uci/#engine-id) command, but the `s` argument
    ///     is `"developer"`, this will error because `id` only has `name` and `author` parameters.
    fn from_message_and_str(
        message_pointer: Self::MessagePointer,
        s: &str,
    ) -> Result<Self, MessageParameterPointerParseError>;
    /// Whether this parameter has a value.
    /// Almost all do, but [`ponder`](https://backscattering.de/chess/uci/#gui-go-ponder)
    /// and [`infinite`](https://backscattering.de/chess/uci/#gui-go-infinite) of the
    /// [`go`](https://backscattering.de/chess/uci/#gui-go) message don't.
    /// They are referred to as "void parameters".
    fn has_value(self) -> bool;
}
