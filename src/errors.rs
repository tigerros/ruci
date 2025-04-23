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
    /// Required parameters are missing.
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
            Self::NoMessage { expected } => write!(f, "missing UCI message, expected {expected}"),
            Self::MissingParameters { expected } => {
                write!(f, "missing UCI parameters, expected {expected}")
            }
            Self::ParameterParseError { expected } => {
                write!(f, "invalid UCI parameters, expected {expected}")
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
            Self::Io(e) => write!(f, "I/O error reading UCI message: {e}"),
            Self::Parse(e) => write!(f, "error parsing UCI message: {e}"),
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
            Self::Write(e) => write!(f, "I/O error sending UCI message: {e}"),
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

#[cfg(all(test, feature = "engine-sync"))]
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
        let message_parse = MessageParseError::NoMessage { expected };
        let read = ReadError::Parse(message_parse);
        let read_str =
            READ_ERROR_PARSE_STR.to_string() + "missing UCI message, expected " + expected;

        assert_eq!(message_parse.expected(), expected);
        assert_eq!(read.to_string(), read_str);
        assert_eq!(
            ReadWriteError::Read(read).to_string(),
            READ_WRITE_ERROR_READ_STR.to_string() + &read_str
        );

        let expected = "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong message cause why noooooooooooooooooot? i'm not doing great. my life is hanging on by a thread, and here i am writing this pointless library. how many times am i going to update it? i wish having OCD would make me organized, but it makes me obsess over things that don't matter. initially working on this made sense, but now it really doesn't. i'm happy doing it, but it's just impulsive. i know i'm neglecting other aspects of my life. if i was sitting on a pile of cash i wouldn't be worried, but i'm not. and here i am, with my dumbass brain doing only what it wants in the short term. not like my future matters. i do think about it, but i don't have the control to stop \"living in the moment\". sounds a lot cooler than it is. maybe some people like it, but my brain is torn between doing what it wants RIGHT NOW, and constantly worrying about the consequences. and i'm told im talented, but my academic performance would beg to differ. \"oh you're so smart\", well i wish i could put it to use. it's so painful to be told something like that only to turn out being a complete failure. i might be smart (or am i??), but i'm stuck in a body which i do not control, so i'm doing much worse than if i was stupid and disciplined. or maybe i'm just stupid and a failure but everyone around says i'm talented. even with a diagnosis and meds i have no control. maybe i belong in an institution.";
        let message_parse = MessageParseError::MissingParameters { expected };
        let read = ReadError::Parse(message_parse);
        let read_str =
            READ_ERROR_PARSE_STR.to_string() + "missing UCI parameters, expected " + expected;

        assert_eq!(message_parse.expected(), expected);
        assert_eq!(read.to_string(), read_str);
        assert_eq!(
            ReadWriteError::Read(read).to_string(),
            READ_WRITE_ERROR_READ_STR.to_string() + &read_str
        );

        let expected = "depth DEEP! <- `Go::from_str` wouldn't actually error given this parameter. maybe i should add a strict mode?";
        let message_parse = MessageParseError::ParameterParseError { expected };
        let read = ReadError::Parse(message_parse);
        let read_str =
            READ_ERROR_PARSE_STR.to_string() + "invalid UCI parameters, expected " + expected;

        assert_eq!(message_parse.expected(), expected);
        assert_eq!(read.to_string(), read_str);
        assert_eq!(
            ReadWriteError::Read(read).to_string(),
            READ_WRITE_ERROR_READ_STR.to_string() + &read_str
        );

        let expected = "later";
        let message_parse = MessageParseError::ValueParseError { expected };
        let read = ReadError::Parse(message_parse);
        let read_str = READ_ERROR_PARSE_STR.to_string() + "invalid UCI value, expected " + expected;

        assert_eq!(message_parse.expected(), expected);
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
