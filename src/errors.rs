use crate::ParameterPointer;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MessageParseError {
    InvalidMessage,
    ParameterParseError(ParameterPointer),
    MissingParameter(ParameterPointer),
    ValueParseError,
    MissingValue,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ParameterPointerParseError {
    MessageHasNoParameters,
    StringDoesNotMapToParameterPointer,
}
