extern crate alloc;

use crate::errors::MessageParseError;
use crate::MessagePointer;
use alloc::string::String;
use core::str::FromStr;

/// Finds the target message and returns the rest of the string, as separated by spaces.
///
/// # Errors
/// No message was found.
pub fn collect_message<'s>(
    target: &'static str,
    s: &'s str,
) -> Result<impl Iterator<Item = &'s str>, MessageParseError>
where
    MessagePointer: FromStr,
{
    let s = s.trim();
    let s = if let Some((s1, _)) = s.split_once('\n') {
        s1
    } else {
        s
    };

    let mut parts = s.split(' ');
    parts
        .find(|&part| part == target)
        .ok_or(MessageParseError::NoMessage { expected: target })?;

    Ok(parts)
}

/// Finds any message and returns the rest of the string, as separated by spaces.
///
/// # Errors
/// No message was found.
pub fn collect_any_message<'s, M>(
    expected: &'static str,
    s: &'s str,
) -> Result<(M, impl Iterator<Item = &'s str>), MessageParseError>
where
    M: FromStr,
{
    let s = s.trim();
    let s = if let Some((s1, _)) = s.split_once('\n') {
        s1
    } else {
        s
    };

    let mut parts = s.split(' ');
    let message = parts
        .find_map(|part| M::from_str(part).ok())
        .ok_or(MessageParseError::NoMessage { expected })?;

    Ok((message, parts))
}

/// Applies a closure to all parameters and their values.
pub fn apply_parameters<'s, ParameterPointer>(
    parts: impl Iterator<Item = &'s str>,
    value: &mut String,
    mut parameter_fn: impl FnMut(ParameterPointer, &str),
) -> &str
where
    ParameterPointer: FromStr,
{
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

    let value = value.trim();

    if let Some(last_parameter) = last_parameter {
        parameter_fn(last_parameter, value);
    }

    value
}
