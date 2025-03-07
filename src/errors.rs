use crate::ParameterPointer;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
#[cfg(feature = "engine-connection")]
use tokio::io;

/// Something went wrong with parsing a message.
///
/// Note that the parsing is pretty liberal and ignores errors unless they're critical.
/// For example, parsing errors will be ignored for most parameters, because they're optional.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MessageParseError {
    /// There either is no message or it's not valid.
    InvalidMessage,
    /// A parameter has an invalid value and could not be parsed (the pointer tells you which one).
    ParameterParseError(ParameterPointer),
    /// A parameter is missing (the pointer tells you which one).
    MissingParameter(ParameterPointer),
    /// The value of the message could not be parsed.
    ValueParseError,
    /// The value is missing.
    MissingValue,
}

impl Display for MessageParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMessage => f.write_str("invalid UCI message"),
            Self::ParameterParseError(p) => {
                write!(f, "could not parse this UCI parameter: {}", p.to_string())
            }
            Self::MissingParameter(p) => write!(f, "missing UCI parameter: {}", p.to_string()),
            Self::ValueParseError => f.write_str("invalid UCI value"),
            Self::MissingValue => f.write_str("missing UCI value"),
        }
    }
}

impl Error for MessageParseError {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ParameterPointerParseError {
    MessageHasNoParameters,
    StringDoesNotMapToParameterPointer,
}

impl Display for ParameterPointerParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringDoesNotMapToParameterPointer => f.write_str("string does not map to parameter pointer"),
            Self::MessageHasNoParameters => f.write_str("message has no parameters and thus can not convert it and a string to a parameter pointer"),
        }
    }
}

impl Error for ParameterPointerParseError {}

#[cfg(feature = "engine-connection")]
#[derive(Debug)]
/// Something went wrong with spawning the engine process.
pub enum CreationError {
    Spawn(io::Error),
    /// See <https://docs.rs/tokio/1.43.0/tokio/process/struct.Child.html#structfield.stdout>.
    StdoutIsNotCaptured,
    /// See <https://docs.rs/tokio/1.43.0/tokio/process/struct.Child.html#structfield.stdin>.
    StdinIsNotCaptured,
}

#[cfg(feature = "engine-connection")]
impl Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spawn(e) => write!(f, "failed to spawn UCI engine connection: {e}"),
            Self::StdoutIsNotCaptured => {
                write!(f, "UCI engine connection process stdout is not captured")
            }
            Self::StdinIsNotCaptured => {
                write!(f, "UCI engine connection process stdin is not captured")
            }
        }
    }
}

#[cfg(feature = "engine-connection")]
impl Error for CreationError {}

#[cfg(feature = "engine-connection")]
#[derive(Debug)]
/// Reading the message either resulted in an IO error, or it could not be parsed.
pub enum ReadMessageError {
    Io(io::Error),
    MessageParse(MessageParseError),
    /// Got GUI message when expecting an engine message.
    GotGuiMessage,
}

#[cfg(feature = "engine-connection")]
impl Display for ReadMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "failed to read UCI engine message: {e}"),
            Self::MessageParse(e) => write!(f, "failed to parse UCI engine message: {e:?}"),
            Self::GotGuiMessage => write!(
                f,
                "received GUI UCI message but was expecting engine message"
            ),
        }
    }
}

#[cfg(feature = "engine-connection")]
impl Error for ReadMessageError {}
