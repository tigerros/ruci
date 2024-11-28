use std::fmt::{Display, Formatter, Write};
use crate::messages::{RawEngineMessage, EngineMessageIdParameterPointer, EngineMessageParameterPointer, EngineMessagePointer};
use crate::{MessageTryFromRawMessageError};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-id>
pub enum Id {
    /// <https://backscattering.de/chess/uci/#engine-id-name>
    Name(String),
    /// <https://backscattering.de/chess/uci/#engine-id-author>
    Author(String),
    NameAndAuthor { name: String, author: String },
}

impl TryFrom<RawEngineMessage> for Id {
    type Error = MessageTryFromRawMessageError<EngineMessageParameterPointer>;

    fn try_from(raw_message: RawEngineMessage) -> Result<Self, Self::Error> {
        if raw_message.message_pointer != EngineMessagePointer::Id {
            return Err(Self::Error::InvalidMessage);
        };

        let name = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Id(
                EngineMessageIdParameterPointer::Name,
            ));

        let author = raw_message
            .parameters
            .get(&EngineMessageParameterPointer::Id(
                EngineMessageIdParameterPointer::Author,
            ));

        #[allow(clippy::option_if_let_else)]
        if let Some(name) = name {
            if let Some(author) = author {
                Ok(Self::NameAndAuthor {
                    name: name.to_string(),
                    author: author.to_string(),
                })
            } else {
                Ok(Self::Name(name.to_string()))
            }
        } else if let Some(author) = author {
            Ok(Self::Author(author.to_string()))
        } else {
            Err(Self::Error::MissingParameter(
                EngineMessageParameterPointer::Id(EngineMessageIdParameterPointer::Name),
            ))
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
    
    use crate::messages::{EngineMessage, Id};

    #[test]
    fn to_from_str() {
        let repr = EngineMessage::Id(Id::NameAndAuthor {
            name: "Stockfish 16.1".to_string(),
            author: "The stockfish developers".to_string(),
        });
        let str_repr = "id name Stockfish 16.1 author The stockfish developers\n";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(EngineMessage::from_str(str_repr), Ok(repr));
    }
}