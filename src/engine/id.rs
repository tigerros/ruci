extern crate alloc;

use alloc::string::String;
use alloc::borrow::{Cow, ToOwned};
use core::fmt::{Display, Formatter};
use crate::engine::pointers::IdParameterPointer;
use crate::errors::MessageParseError;
use crate::dev_macros::{from_str_parts, impl_message, message_from_impl};
use crate::parsing;
use super::{pointers, traits};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Engine's identification information.
/// 
/// Sent after [`Uci`](crate::gui::Uci).
///
/// <https://backscattering.de/chess/uci/#engine-id>
pub enum Id<'a> {
    /// <https://backscattering.de/chess/uci/#engine-id-name>
    Name(Cow<'a, str>),
    /// <https://backscattering.de/chess/uci/#engine-id-author>
    Author(Cow<'a, str>),
    NameAndAuthor { name: Cow<'a, str>, author: Cow<'a, str> },
}

impl Id<'_> {
    #[must_use]
    /// Returns a new [`Id`] that is updated with values of the `new` parameter.
    ///
    /// # Examples
    /// ```rust
    /// # use pretty_assertions::assert_eq;
    /// # use ruci::engine::Id;
    /// let mut id = Id::Name("Carp".into());
    ///
    /// id = id.updated(Id::Name("Salmon".into()));
    /// assert_eq!(id, Id::Name("Salmon".into()));
    ///
    /// id = id.updated(Id::Author("Fischerman".into()));
    /// assert_eq!(
    ///     id,
    ///     Id::NameAndAuthor {
    ///         name: "Salmon".into(),
    ///         author: "Fischerman".into()
    ///     }
    /// );
    ///
    /// id = id.updated(Id::Author("Garry Chess".into()));
    /// assert_eq!(
    ///     id,
    ///     Id::NameAndAuthor {
    ///         name: "Salmon".into(),
    ///         author: "Garry Chess".into()
    ///     }
    /// );
    ///
    /// id = id.updated(Id::Name("Big Tuna".into()));
    /// assert_eq!(
    ///     id,
    ///     Id::NameAndAuthor {
    ///         name: "Big Tuna".into(),
    ///         author: "Garry Chess".into()
    ///     }
    /// );
    /// ```
    pub fn updated(self, new: Self) -> Self {
        match (self, new) {
            (Id::Author(_), Id::Author(author)) => Id::Author(author),
            (Id::Name(_), Id::Name(name)) => Id::Name(name),
            (
                Id::NameAndAuthor { .. } | Id::Author(_) | Id::Name(_),
                Id::NameAndAuthor { name, author },
            )
            | (Id::NameAndAuthor { author: _, name } | Id::Name(name), Id::Author(author))
            | (Id::NameAndAuthor { author, name: _ } | Id::Author(author), Id::Name(name)) => {
                Id::NameAndAuthor { name, author }
            }
        }
    }
}

impl_message!(Id<'_>);
message_from_impl!(engine Id<'a>);
from_str_parts!(impl Id<'a> for parts -> Result {
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
                name: Cow::Owned(name),
                author: Cow::Owned(author),
            })
        } else {
            Ok(Self::Name(Cow::Owned(name)))
        }
    } else if let Some(author) = author {
        Ok(Self::Author(Cow::Owned(author)))
    } else {
        Err(MessageParseError::MissingParameters { expected: "name or author" })
    }
});

impl Display for Id<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("id ")?;

        match self {
            Self::Name(name) => write!(f, "name {name}")?,
            Self::Author(author) => write!(f, "author {author}")?,
            Self::NameAndAuthor { name, author } => {
                write!(f, "name {name} author {author}")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;
    use core::str::FromStr;
    use pretty_assertions::assert_eq;
    use crate::{engine, Message};
    use super::Id;
    use alloc::string::ToString;

    #[test]
    fn to_from_str() {
        let repr: Message = Id::NameAndAuthor {
            name: Cow::from("Stockfish 16.1"),
            author: Cow::from("The stockfish developers"),
        }.into();
        let str_repr = "id name Stockfish 16.1 author The stockfish developers";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(Message::from_str(str_repr), Ok(repr));

        let repr: engine::Message = Id::Name(Cow::from("Stockfish 16.1")).into();
        let str_repr = "id name Stockfish 16.1";

        assert_eq!(repr.to_string(), str_repr);
        assert_eq!(engine::Message::from_str(str_repr), Ok(repr));
    }
}