use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
#[cfg(feature = "io")]
use std::io;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageParseErrorKind {
    /// No message in the string was found.
    NoMessage,
    /// Required parameters are missing.
    MissingParameters,
    /// A required parameter could not be parsed.
    ParameterParseError,
    /// The required value of the message could not be parsed.
    ValueParseError,
}

/// Something went wrong with parsing a message.
///
/// Note that the parsing is very liberal and ignores errors unless they're critical.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageParseError {
    pub expected: &'static str,
    pub kind: MessageParseErrorKind,
}

impl Display for MessageParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let expected = self.expected;

        match self.kind {
            MessageParseErrorKind::NoMessage => {
                write!(f, "missing UCI message, expected {expected}")
            }
            MessageParseErrorKind::MissingParameters => {
                write!(f, "missing UCI parameters, expected {expected}")
            }
            MessageParseErrorKind::ParameterParseError => {
                write!(f, "invalid UCI parameters, expected {expected}")
            }
            MessageParseErrorKind::ValueParseError => {
                write!(f, "invalid UCI value, expected {expected}")
            }
        }
    }
}

impl Error for MessageParseError {}

#[cfg(feature = "io")]
#[cfg_attr(docsrs, doc(cfg(feature = "io")))]
#[derive(Debug)]
/// Reading a message from the engine failed.
pub enum ReadError {
    /// Reading failed due to an I/O error.
    Io(io::Error),
    /// Reading succeeded but parsing to a [`engine::Message`](crate::engine::Message) failed.
    Parse {
        error: MessageParseError,
        /// The erroneous string. Includes the newline character (`\n`).
        got: String,
    },
}

#[cfg(feature = "io")]
impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error reading UCI message: {e}"),
            Self::Parse { error, got } => {
                write!(f, "error parsing UCI message: {error}. Got: {got}")
            }
        }
    }
}

#[cfg(feature = "io")]
impl Error for ReadError {}

#[cfg(feature = "io")]
#[cfg_attr(docsrs, doc(cfg(feature = "io")))]
#[derive(Debug)]
/// Reading/sending a message from/to the engine failed.
pub enum ReadWriteError {
    /// Sending a message failed due to an I/O error.
    Write(io::Error),
    /// Reading a message failed.
    Read(ReadError),
}

#[cfg(feature = "io")]
impl Display for ReadWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Write(e) => write!(f, "I/O error sending UCI message: {e}"),
            Self::Read(e) => write!(f, "error reading UCI message: {e}"),
        }
    }
}

#[cfg(feature = "io")]
impl Error for ReadWriteError {}

#[cfg(feature = "io")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Converting a process to an engine failed.
pub enum FromProcessError {
    /// See [`Child.stdout`](std::process::Child#structfield.stdout).
    StdoutNotCaptured,
    /// See [`Child.stdin`](std::process::Child#structfield.stdin).
    StdinNotCaptured,
}

#[cfg(feature = "io")]
impl Display for FromProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdoutNotCaptured => write!(f, "UCI process stdout is not captured"),
            Self::StdinNotCaptured => write!(f, "UCI process stdin is not captured"),
        }
    }
}

#[cfg(feature = "io")]
impl Error for FromProcessError {}

#[cfg(all(test, feature = "io"))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn read_write() {
        const READ_ERROR_PARSE_STR: &str = "error parsing UCI message: ";
        const READ_WRITE_ERROR_READ_STR: &str = "error reading UCI message: ";
        let io_str = "screw your connection";
        assert_eq!(
            ReadWriteError::Write(io::Error::new(io::ErrorKind::ConnectionAborted, io_str))
                .to_string(),
            "I/O error sending UCI message: ".to_string() + io_str
        );

        let io_str = "i feel like the ErrorKind should be shown too :/";
        let read = ReadError::Io(io::Error::other(io_str));
        let read_str = "I/O error reading UCI message: ".to_string() + io_str;

        assert_eq!(read.to_string(), read_str);
        assert_eq!(
            ReadWriteError::Read(read).to_string(),
            READ_WRITE_ERROR_READ_STR.to_string() + &read_str
        );

        let expected = "apologies for being informal in these tests 😔 <- UTF-8 test right there, very serious!";
        let message_parse = MessageParseError {
            expected,
            kind: MessageParseErrorKind::NoMessage,
        };
        let read = ReadError::Parse {
            error: message_parse,
            got: "got".to_string(),
        };
        let read_str = READ_ERROR_PARSE_STR.to_string()
            + "missing UCI message, expected "
            + expected
            + ". Got: got";

        assert_eq!(message_parse.expected, expected);
        assert_eq!(read.to_string(), read_str);
        assert_eq!(
            ReadWriteError::Read(read).to_string(),
            READ_WRITE_ERROR_READ_STR.to_string() + &read_str
        );

        let expected = "�������";
        let message_parse = MessageParseError {
            expected,
            kind: MessageParseErrorKind::MissingParameters,
        };
        let read = ReadError::Parse {
            error: message_parse,
            got: String::new(),
        };
        let read_str = READ_ERROR_PARSE_STR.to_string()
            + "missing UCI parameters, expected "
            + expected
            + ". Got: ";

        assert_eq!(message_parse.expected, expected);
        assert_eq!(read.to_string(), read_str);
        assert_eq!(
            ReadWriteError::Read(read).to_string(),
            READ_WRITE_ERROR_READ_STR.to_string() + &read_str
        );

        let expected = "depth DEEP! <- `Go::from_str` wouldn't actually error given this parameter. maybe i should add a strict mode?";
        let message_parse = MessageParseError {
            expected,
            kind: MessageParseErrorKind::ParameterParseError,
        };
        let read = ReadError::Parse {
            error: message_parse,
            got: " t".to_string(),
        };
        let read_str = READ_ERROR_PARSE_STR.to_string()
            + "invalid UCI parameters, expected "
            + expected
            + ". Got:  t";

        assert_eq!(message_parse.expected, expected);
        assert_eq!(read.to_string(), read_str);
        assert_eq!(
            ReadWriteError::Read(read).to_string(),
            READ_WRITE_ERROR_READ_STR.to_string() + &read_str
        );

        let expected = "later";
        let message_parse = MessageParseError {
            expected,
            kind: MessageParseErrorKind::ValueParseError,
        };
        let read = ReadError::Parse {
            error: message_parse,
            got: " ".to_string(),
        };
        let read_str = READ_ERROR_PARSE_STR.to_string()
            + "invalid UCI value, expected "
            + expected
            + ". Got:  ";

        assert_eq!(message_parse.expected, expected);
        assert_eq!(read.to_string(), read_str);
        assert_eq!(
            ReadWriteError::Read(read).to_string(),
            READ_WRITE_ERROR_READ_STR.to_string() + &read_str
        );
    }

    #[test]
    fn from_process() {
        assert_eq!(
            FromProcessError::StdoutNotCaptured.to_string(),
            "UCI process stdout is not captured"
        );
        assert_eq!(
            FromProcessError::StdinNotCaptured.to_string(),
            "UCI process stdin is not captured"
        );
    }
}
