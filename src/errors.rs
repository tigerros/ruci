use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
#[cfg(feature = "engine-sync")]
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

#[cfg(feature = "engine-sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "engine-sync")))]
#[derive(Debug)]
/// Reading a message from the engine failed.
pub enum ReadError {
    /// Reading failed due to an I/O error.
    Io(io::Error),
    /// Reading succeeded but parsing to a [`engine::Message`](crate::engine::Message) failed.
    Parse(MessageParseError),
}

#[cfg(feature = "engine-sync")]
impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error while trying to read UCI message: {e}"),
            Self::Parse(e) => write!(f, "failed to parse UCI message: {e}"),
        }
    }
}

#[cfg(feature = "engine-sync")]
impl Error for ReadError {}

#[cfg(feature = "engine-sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "engine-sync")))]
#[derive(Debug)]
/// Reading/sending a message from/to the engine failed.
pub enum ReadWriteError {
    /// Sending a message failed due to an I/O error.
    Write(io::Error),
    /// Reading a message failed.
    Read(ReadError),
}

#[cfg(feature = "engine-sync")]
impl Display for ReadWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Write(e) => write!(f, "I/O error while trying to sending UCI message: {e}"),
            Self::Read(e) => write!(f, "error reading UCI message: {e}"),
        }
    }
}

#[cfg(feature = "engine-sync")]
impl Error for ReadWriteError {}

#[cfg(feature = "engine-sync")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FromProcessError {
    /// See [`Child.stdout`](std::process::Child#structfield.stdout).
    StdoutNotCaptured,
    /// See [`Child.stdin`](std::process::Child#structfield.stdin).
    StdinNotCaptured,
}

#[cfg(feature = "engine-sync")]
impl Display for FromProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdoutNotCaptured => write!(f, "UCI process stdout is not captured"),
            Self::StdinNotCaptured => write!(f, "UCI process stdin is not captured"),
        }
    }
}

#[cfg(feature = "engine-sync")]
impl Error for FromProcessError {}
