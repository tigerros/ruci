use std::fmt::{Display, Formatter, Write};
use crate::errors::MessageParseError;
use crate::from_str_parts::from_str_parts;
use crate::gui::pointers::RegisterParameterPointer;
use crate::message_from_impl::message_from_impl;
use crate::parsing;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// <https://backscattering.de/chess/uci/#gui-register>
pub enum Register {
    Later,
    Name(String),
    Code(String),
    NameAndCode { name: String, code: String },
}

message_from_impl!(gui Register);
from_str_parts!(impl Register for parts {
    let mut value = String::with_capacity(50);
    let mut last_parameter = None::<RegisterParameterPointer>;
    let mut name = None;
    let mut code = None;
    let mut parameter_to_closure = |parameter, value: &str| match parameter {
        RegisterParameterPointer::Name => name = Some(value.to_owned()),
        RegisterParameterPointer::Code => code = Some(value.to_owned()),
    };
    let mut first_parameter_encountered = false;
    
    for part in parts {
        let Some(parameter) = parsing::get_parameter_or_update_value(part, &mut value) else {
            continue;
        };

        if !first_parameter_encountered {
            value.clear();
            first_parameter_encountered = true;
        }

        if let Some(last_parameter) = last_parameter {
            parameter_to_closure(last_parameter, value.trim());
            value.clear();
        }

        last_parameter = Some(parameter);
    }

    if let Some(last_parameter) = last_parameter {
        parameter_to_closure(last_parameter, value.trim());
    }

    if let Some(name) = name {
        if let Some(code) = code {
            Ok(Self::NameAndCode {
                name,
                code,
            })
        } else {
            Ok(Self::Name(name))
        }
    } else if let Some(code) = code {
        Ok(Self::Code(code))
    } else if value.trim() == "later" {
        Ok(Self::Later)
    } else {
        Err(MessageParseError::MissingParameter(RegisterParameterPointer::Name.into()))
    }
});

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Later => f.write_str("register later")?,
            Self::Name(name) => write!(f, "register name {name}")?,
            Self::Code(code) => write!(f, "register code {code}")?,
            Self::NameAndCode { name, code } => {
                write!(f, "register name {name} code {code}")?;
            }
        }
        
        f.write_char('\n')
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::str::FromStr;
    use pretty_assertions::assert_eq;
    use crate::gui::Register;
    use crate::Message;

    #[test]
    fn to_from_str() {
        let m: Message = Register::NameAndCode {
            name: "john smith".to_string(),
            code: "31 tango".to_string()
        }.into();
        let str = "register name john smith code 31 tango\n";

        assert_eq!(m.to_string(), str);
        assert_eq!(Message::from_str(str), Ok(m));
    }

    #[test]
    fn invalid_parameter() {
        let m: Message = Register::Name("a l o t o f s p a c e s".to_string()).into();
        assert_eq!(m.to_string(), "register name a l o t o f s p a c e s\n");
        assert_eq!(Message::from_str("register blahblah woo name a l o t o f s p a c e s\n").unwrap(), m);
    }
}