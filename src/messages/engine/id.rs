use std::fmt::{Display, Formatter, Write};
use crate::messages::{BestMoveMessage, EngineMessage};
use crate::messages::engine::{EngineMessageBestMoveParameterPointer, EngineMessageIdParameterPointer, EngineMessageParameterPointer, EngineMessagePointer};
use crate::{MessageTryFromRawUciMessageError, RawUciMessage};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// <https://backscattering.de/chess/uci/#engine-id>
pub enum IdMessageKind {
    /// <https://backscattering.de/chess/uci/#engine-id-name>
    Name(String),
    /// <https://backscattering.de/chess/uci/#engine-id-author>
    Author(String),
    NameAndAuthor(String, String),
}

impl TryFrom<RawUciMessage<EngineMessage>> for IdMessageKind {
    type Error = MessageTryFromRawUciMessageError<EngineMessageParameterPointer>;

    fn try_from(raw_uci_message: RawUciMessage<EngineMessage>) -> Result<Self, Self::Error> {
        if raw_uci_message.message_pointer != EngineMessagePointer::Id {
            return Err(Self::Error::InvalidMessage);
        };

        let name = raw_uci_message
            .parameters
            .get(&EngineMessageParameterPointer::Id(
                EngineMessageIdParameterPointer::Name,
            ));

        let author = raw_uci_message
            .parameters
            .get(&EngineMessageParameterPointer::Id(
                EngineMessageIdParameterPointer::Author,
            ));

        #[allow(clippy::option_if_let_else)]
        if let Some(name) = name {
            if let Some(author) = author {
                Ok(Self::NameAndAuthor(
                    name.to_string(),
                    author.to_string(),
                ))
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

impl Display for IdMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("id ")?;

        match self {
            IdMessageKind::Name(name) => write!(f, "name {name}")?,
            IdMessageKind::Author(author) => write!(f, "author {author}")?,
            IdMessageKind::NameAndAuthor(name, author) => {
                write!(f, "name {name} author {author}")?;
            }
        }

        f.write_char('\n')
    }
}