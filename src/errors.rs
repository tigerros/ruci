use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
#[cfg(feature = "engine-connection")]
use tokio::io;

/// Something went wrong with parsing a message.
///
/// Note that the parsing is pretty liberal and ignores errors unless they're critical.
/// For example, parsing errors will be ignored for most parameters, because they're optional.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageParseError {
    /// No message in the string was found.
    NoMessage {
        expected: &'static str,
    },
    /// Required parameter(s) are missing.
    MissingParameters {
        expected: &'static str,
    },
    /// A required parameter could not be parsed.
    ParameterParseError {
        expected: &'static str,
    },
    /// The required value of the message could not be parsed.
    ValueParseError {
        expected: &'static str,
    },
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

#[cfg(feature = "engine-connection")]
#[derive(Debug)]
/// Something went wrong with spawning the engine process.
pub enum ConnectionError {
    Spawn(io::Error),
    /// See <https://docs.rs/tokio/1.44.1/tokio/process/struct.Child.html#structfield.stdout>.
    StdoutIsNotCaptured,
    /// See <https://docs.rs/tokio/1.44.1/tokio/process/struct.Child.html#structfield.stdin>.
    StdinIsNotCaptured,
}

#[cfg(feature = "engine-connection")]
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

#[cfg(feature = "engine-connection")]
impl Error for ConnectionError {}

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "failed to read UCI engine message: {e}"),
            Self::MessageParse(e) => write!(f, "failed to parse UCI engine message: {e}"),
            Self::GotGuiMessage => write!(
                f,
                "received GUI UCI message but was expecting engine message"
            ),
        }
    }
}

#[cfg(feature = "engine-connection")]
impl Error for ReadMessageError {}
