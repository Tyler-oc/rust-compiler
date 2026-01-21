use std::error::Error;
use std::fmt;

use crate::errors::interpreter_error::InterpreterError;
use crate::errors::lex_error::LexError;
use crate::errors::parse_error::ParseError;

#[derive(Debug)]
pub enum GeneralError {
    LexError(LexError),
    ParseError(ParseError),
    InterpreterError(InterpreterError),
}

impl std::fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneralError::LexError(e) => write!(f, "{}", e),
            GeneralError::ParseError(e) => write!(f, "{}", e),
            GeneralError::InterpreterError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for GeneralError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GeneralError::LexError(e) => Some(e),
            GeneralError::ParseError(e) => Some(e),
            GeneralError::InterpreterError(e) => Some(e),
        }
    }
}

impl From<LexError> for GeneralError {
    fn from(value: LexError) -> Self {
        GeneralError::LexError(value)
    }
}

impl From<ParseError> for GeneralError {
    fn from(value: ParseError) -> Self {
        GeneralError::ParseError(value)
    }
}

impl From<InterpreterError> for GeneralError {
    fn from(value: InterpreterError) -> Self {
        GeneralError::InterpreterError(value)
    }
}
