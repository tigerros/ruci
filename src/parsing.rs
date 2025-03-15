use crate::{MessagePointer};
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use crate::engine::pointers::OptionParameterPointer;
use crate::errors::MessageParseError;

/// Finds the target message and returns the rest of the string, as separated by spaces.
///
/// # Errors
/// No message was found.
pub fn collect_message<'s>(target: &str, s: &'s str) -> Result<impl Iterator<Item = &'s str>, MessageParseError> where MessagePointer: FromStr {
    let s = s.trim();
    let s = if let Some((s1, _)) = s.split_once('\n') {
        s1
    } else {
        s
    };

    let mut parts = s.split(' ');
    parts
        .find(|&part| part == target)
        .ok_or(MessageParseError::InvalidMessage)?;

    Ok(parts)
}

/// Finds any message and returns the rest of the string, as separated by spaces.
///
/// # Errors
/// No message was found.
pub fn collect_any_message(s: &str) -> Result<(MessagePointer, impl Iterator<Item = &str>), MessageParseError> where MessagePointer: FromStr {
    let s = s.trim();
    let s = if let Some((s1, _)) = s.split_once('\n') {
        s1
    } else {
        s
    };

    let mut parts = s.split(' ');
    let message = parts
        .find_map(|part| MessagePointer::from_str(part).ok())
        .ok_or(MessageParseError::InvalidMessage)?;

    Ok((message, parts))
}

pub fn get_parameter_or_update_value<ParameterPointer>(part: &str, value: &mut String) -> Option<ParameterPointer> where ParameterPointer: FromStr {
    ParameterPointer::from_str(part).map_or_else(|_| {
        value.push_str(part);
        value.push(' ');
        None
    }, Some)
}

/// Applies a closure to all parameters and their values.
pub fn apply_parameters<'s, ParameterPointer>(parts: impl Iterator<Item = &'s str>, parameter_fn: fn(ParameterPointer, &str)) where ParameterPointer: FromStr {
    let mut value = String::with_capacity(50);
    let mut first_parameter_encountered = false;
    let mut last_parameter = None;

    for part in parts {
        let Ok(parameter) = ParameterPointer::from_str(part) else {
            if first_parameter_encountered {
                value.push_str(part);
                value.push(' ');
            }
            continue;
        };

        if !first_parameter_encountered {
            first_parameter_encountered = true;
        }

        if let Some(last_parameter) = last_parameter {
            parameter_fn(last_parameter, value.trim());
            value.clear();
        }

        last_parameter = Some(parameter);
    }

    if let Some(last_parameter) = last_parameter {
        parameter_fn(last_parameter, value.trim());
    }
}

/// Use at the end of parsing to avoid unnecessarily clearing the value.
pub fn insert_last_parameter<ParameterPointer>(last_parameter: ParameterPointer, parameters: &mut HashMap<ParameterPointer, String>, value: &String)
where ParameterPointer: FromStr + Eq + Hash {
    parameters.insert(last_parameter, value.trim().to_string());
}

/// Specialized [`insert_last_parameter`] for the `option` message.
pub fn option_insert_last_parameter(last_parameter: OptionParameterPointer, parameters: &mut HashMap<OptionParameterPointer, String>, option_vars: &mut Vec<String>, value: &String) {
    if last_parameter == OptionParameterPointer::Var {
        option_vars.push(value.trim().to_string());
    } else {
        parameters.insert(last_parameter, value.trim().to_string());
    }
}

/// Use in the parsing loop if `last_parameter` is some.
pub fn insert_last_parameter_and_clear_value<ParameterPointer>(last_parameter: ParameterPointer, parameters: &mut HashMap<ParameterPointer, String>, value: &mut String)
where ParameterPointer: FromStr + Eq + Hash {
    insert_last_parameter(last_parameter, parameters, value);
    value.clear();
}

/// Specialized [`insert_last_parameter_and_clear_value`] for the `option` message.
pub fn option_insert_last_parameter_and_clear_value(last_parameter: OptionParameterPointer, parameters: &mut HashMap<OptionParameterPointer, String>, option_vars: &mut Vec<String>, value: &mut String) {
    option_insert_last_parameter(last_parameter, parameters, option_vars, value);
    value.clear();
}