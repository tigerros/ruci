use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use crate::errors::MessageParseError;
use crate::{MessagePointer};

/// Verifies that the string contains the given message pointer and returns the fragments split by
/// spaces after the message string.
pub fn verify_and_get_parts(target_message_pointer: MessagePointer, s: &str) -> Result<impl Iterator<Item = &str>, MessageParseError> {
    let s = s.trim();
    let s = if let Some((s1, _)) = s.split_once('\n') {
        s1
    } else {
        s
    };

    let mut parts = s.split(' ');
    let message_pointer = parts.find_map(|part| MessagePointer::from_str(part).ok()).ok_or(MessageParseError::InvalidMessage)?;

    if message_pointer != target_message_pointer {
        return Err(MessageParseError::InvalidMessage);
    }

    Ok(parts)
}

pub fn init_parameters<P>(parts_len: usize) -> HashMap<P, String> {
    HashMap::with_capacity(parts_len.saturating_div(2))
}

/// Treat this like a function with `part: &str, value: &mut String` params,
/// but prefix it with the parameter pointer type you want.
///
/// Tries to part the given `part` to a parameter pointer, if that fails adds the `part` to the `value`.
macro_rules! get_parameter_pointer_or_update_value {
    ($parameter_pointer:ty: $part:expr, $value:expr) => {{
        if let Some(p) = <$parameter_pointer>::from_str($part) {
            Some(p)
        } else {
            $value.push_str($part);
            $value.push(' ');
            None
        }
    }};
}

pub(crate) use get_parameter_pointer_or_update_value;

// pub fn get_parameter_pointer_or_update_value(message_pointer: MessagePointer, part: &str, value: &mut String) -> Option<ParameterPointer> {
//     ParameterPointer::from_message_and_str(message_pointer, part).map_or_else(|_| {
//         value.push_str(part);
//         value.push(' ');
//         None
//     }, Some)
// }

pub fn update_parameters_if_last_parameter_is_some<P>(last_parameter: Option<P>, value: &mut String, parameters: &mut HashMap<P, String>) where P: Hash + Eq {
    if let Some(last_parameter) = last_parameter {
        parameters.insert(last_parameter, value.trim().to_string());

        *value = String::with_capacity(50);
    }
}