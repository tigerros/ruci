use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use crate::engine::pointers::IdParameterPointer;
use crate::errors::MessageParseError;
use crate::from_str_parts::from_str_parts;
use crate::message_from_impl::message_from_impl;
use crate::parsing;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#engine-id>
pub enum Id {
    /// <https://backscattering.de/chess/uci/#engine-id-name>
    Name(String),
    /// <https://backscattering.de/chess/uci/#engine-id-author>
    Author(String),
    NameAndAuthor { name: String, author: String },
}

message_from_impl!(engine Id);
from_str_parts!(impl Id for parts {
    let mut name = None::<String>;
    let mut author = None::<String>;
    let mut value = String::with_capacity(50);
    let mut last_parameter = None::<IdParameterPointer>;
    let mut parameter_to_closure = |parameter, value: &str| match parameter {
        IdParameterPointer::Name => name = Some(value.to_owned()),
        IdParameterPointer::Author => author = Some(value.to_owned()),
    };
    
    for part in parts {
        let Some(parameter) = parsing::get_parameter_or_update_value(part, &mut value) else {
            continue;
        };
        if let Some(last_parameter) = last_parameter {
            parameter_to_closure(last_parameter, value.trim());
            value.clear();
        }
        last_parameter = Some(parameter);
    }

    if let Some(last_parameter) = last_parameter {
        parameter_to_closure(last_parameter, value.trim());
    }

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
        Err(MessageParseError::MissingParameter(IdParameterPointer::Name.into()))
    }
});

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