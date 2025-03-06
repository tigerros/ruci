use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::message_from_impl::message_from_impl;
use crate::raw_message::RawMessage;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// <https://backscattering.de/chess/uci/#engine-id>
pub enum Id {
    /// <https://backscattering.de/chess/uci/#engine-id-name>
    Name(String),
    /// <https://backscattering.de/chess/uci/#engine-id-author>
    Author(String),
    NameAndAuthor { name: String, author: String },
}

message_from_impl!(engine Id);

impl TryFrom<RawMessage> for Id {
    type Error = MessageParseError;

    fn try_from(mut raw_message: RawMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != super::pointers::MessagePointer::Id.into() {
            return Err(Self::Error::InvalidMessage);
        };

        let name = raw_message
            .parameters
            .remove(&super::pointers::IdParameterPointer::Name.into());

        let author = raw_message
            .parameters
            .remove(&super::pointers::IdParameterPointer::Author.into());

        #[allow(clippy::option_if_let_else)]
        if let Some(name) = name {
            if let Some(author) = author {
                Ok(Self::NameAndAuthor {
                    name,
                    author,
                })
            } else {
                Ok(Self::Name(name))
            }
        } else if let Some(author) = author {
            Ok(Self::Author(author))
        } else {
            Err(Self::Error::MissingParameter(super::pointers::IdParameterPointer::Name.into()))
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("id ")?;

        match self {
            Self::Name(name) => write!(f, "name {name}")?,
            Self::Author(author) => write!(f, "author {author}")?,
            Self::NameAndAuthor { name, author } => {
                write!(f, "name {name} author {author}")?;
            }
        }

        f.write_char('\n')
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use pretty_assertions::assert_eq;
    use crate::Message;
    use super::Id;

    #[test]
    fn to_from_str() {
        let repr: Message = Id::NameAndAuthor {
            name: "Stockfish 16.1".to_string(),
            author: "The stockfish developers".to_string(),
        }.into();
        let str_repr = "id name Stockfish 16.1 author The stockfish developers\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));
    }
}