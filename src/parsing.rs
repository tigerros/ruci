extern crate alloc;

use crate::errors::MessageParseError;
use crate::{MessageParseErrorKind, MessagePointer};
use alloc::string::String;
use core::str::FromStr;

/// Finds the target message and returns the rest of the string, as separated by spaces.
///
/// # Errors
/// No message was found.
pub fn collect_message<'s>(
    target: &'static str,
    s: &'s str,
) -> Result<core::str::Split<'s, char>, MessageParseError>
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
        .ok_or(MessageParseError {
            expected: target,
            kind: MessageParseErrorKind::NoMessage,
        })?;

    Ok(parts)
}

/// Finds any message and returns the rest of the string, as separated by spaces.
///
/// # Errors
/// No message was found.
pub fn collect_any_message<'s, M>(
    expected: &'static str,
    s: &'s str,
) -> Result<(M, core::str::Split<'s, char>), MessageParseError>
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
        .ok_or(MessageParseError {
            expected,
            kind: MessageParseErrorKind::NoMessage,
        })?;

    Ok((message, parts))
}

/// Applies a closure to all parameters and their values.
///
/// `parameter_fn` accepts these parameters:
/// - `ParameterPointer`: the pointer associated with the passed in value.
/// - `Option<ParameterPointer>`: the next parameter, if there is one.
/// - `Split<char>`: the parts that this function parses. You should return these, as is given
///   by the `parameter_fn` signature, unless you want to stop parsing. In that case, return `None`
///   and keep the parts to yourself.
pub fn apply_parameters<'p, 'v, ParameterPointer>(
    mut parts: core::str::Split<'p, char>,
    value: &'v mut String,
    mut parameter_fn: impl FnMut(
        ParameterPointer,
        Option<ParameterPointer>,
        &str,
        core::str::Split<'p, char>,
    ) -> Option<core::str::Split<'p, char>>,
) -> Option<&'v str>
where
    ParameterPointer: FromStr + Copy,
{
    let mut first_parameter_encountered = false;
    let mut last_parameter = None;

    while let Some(part) = parts.next() {
        let Ok(parameter) = ParameterPointer::from_str(part) else {
            value.push_str(part);
            value.push(' ');
            continue;
        };

        if !first_parameter_encountered {
            first_parameter_encountered = true;
            value.clear();
        }

        if let Some(last_parameter) = last_parameter {
            if let Some(p) = parameter_fn(last_parameter, Some(parameter), value.trim(), parts) {
                parts = p;
            } else {
                return None;
            }

            value.clear();
        }

        last_parameter = Some(parameter);
    }

    let value = value.trim();

    if let Some(last_parameter) = last_parameter {
        parameter_fn(last_parameter, None, value, parts);
    }

    Some(value)
}
