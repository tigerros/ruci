use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
#[cfg(feature = "std")]
use std::io;

/// Something went wrong with parsing a message.
///
/// Note that the parsing is very liberal and ignores errors unless they're critical.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageParseError {
    /// No message in the string was found.
    NoMessage { expected: &'static str },
    /// Required parameter(s) are missing.
    MissingParameters { expected: &'static str },
    /// A required parameter could not be parsed.
    ParameterParseError { expected: &'static str },
    /// The required value of the message could not be parsed.
    ValueParseError { expected: &'static str },
}

impl MessageParseError {
    pub const fn expected(self) -> &'static str {
        match self {
            Self::NoMessage { expected }
            | Self::MissingParameters { expected }
            | Self::ParameterParseError { expected }
            | Self::ValueParseError { expected } => expected,
        }
    }
}

impl Display for MessageParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoMessage { expected } => write!(f, "invalid UCI message, expected {expected}"),
            Self::MissingParameters { expected } => {
                write!(f, "missing UCI parameter, expected {expected}")
            }
            Self::ParameterParseError { expected } => {
                write!(f, "invalid UCI parameter, expected {expected}")
            }
            Self::ValueParseError { expected } => {
                write!(f, "invalid UCI value, expected {expected}")
            }
        }
    }
}

impl Error for MessageParseError {}

#[cfg(feature = "std")]
#[derive(Debug)]
/// Initiating the engine process failed.
pub enum ConnectionError {
    Spawn(io::Error),
    /// See [`tokio::process::Child.stdout`](tokio::process::Child#structfield.stdout).
    StdoutIsNotCaptured,
    /// See [`tokio::process::Child.stdin`](tokio::process::Child#structfield.stdin).
    StdinIsNotCaptured,
}

#[cfg(feature = "std")]
impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

#[cfg(feature = "std")]
impl Error for ConnectionError {}

#[cfg(feature = "std")]
#[derive(Debug)]
/// Reading a message from the engine failed.
pub enum ReadError {
    /// Reading failed due to an I/O error.
    Io(io::Error),
    /// Reading succeeded but parsing to a [`engine::Message`](crate::engine::Message) failed.
    Parse(MessageParseError),
}

#[cfg(feature = "std")]
impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error while trying to read UCI message: {e}"),
            Self::Parse(e) => write!(f, "failed to parse UCI message: {e}"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ReadError {}

#[cfg(feature = "std")]
#[derive(Debug)]
/// Reading/sending a message from/to the engine failed.
pub enum ReadWriteError {
    /// Sending a message failed due to an I/O error.
    Write(io::Error),
    /// Reading a message failed.
    Read(ReadError),
}

#[cfg(feature = "std")]
impl Display for ReadWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Write(e) => write!(f, "I/O error while trying to sending UCI message: {e}"),
            Self::Read(e) => write!(f, "error reading UCI message: {e}"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ReadWriteError {}
