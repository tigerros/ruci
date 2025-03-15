extern crate alloc;

use alloc::string::String;
use alloc::borrow::ToOwned;
use core::fmt::{Display, Formatter, Write};
use crate::engine::pointers::IdParameterPointer;
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, message_from_impl};
use crate::parsing;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Engine's identification information.
/// 
/// Sent after [`Uci`](crate::gui::Uci).
///
/// <https://backscattering.de/chess/uci/#engine-id>
pub enum Id {
    /// <https://backscattering.de/chess/uci/#engine-id-name>
    Name(String),
    /// <https://backscattering.de/chess/uci/#engine-id-author>
    Author(String),
    NameAndAuthor { name: String, author: String },
}

message_from_impl!(engine Id);
from_str_parts!(impl Id for parts -> Result<Self, MessageParseError>  {
    let mut name = None::<String>;
    let mut author = None::<String>;
    let parameter_fn = |parameter, value: &str| match parameter {
        IdParameterPointer::Name => name = Some(value.to_owned()),
        IdParameterPointer::Author => author = Some(value.to_owned()),
    };
    
    let mut value = String::with_capacity(200);
    parsing::apply_parameters(parts, &mut value, parameter_fn);

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
        Err(MessageParseError::MissingParameters { expected: "name or author" })
    }
});

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
    use core::str::FromStr;
    use pretty_assertions::assert_eq;
    use crate::Message;
    use super::Id;
    use alloc::string::ToString;

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