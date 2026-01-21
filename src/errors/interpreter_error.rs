use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub enum InterpreterError {
    CouldNotEval(String),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::CouldNotEval(val) => write!(f, "Could not evaluate: {}", val),
        }
    }
}

impl Error for InterpreterError {}
