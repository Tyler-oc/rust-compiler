use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EnvironmentError {
    MultipleAssignmentVariable(String),
    UndefinedVariable(String),
}

impl std::fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvironmentError::MultipleAssignmentVariable(name) => {
                write!(
                    f,
                    "variable assigned more than once in same scope: {}",
                    name
                )
            }
            EnvironmentError::UndefinedVariable(name) => {
                write!(f, "undefined variable: {}", name)
            }
        }
    }
}

impl Error for EnvironmentError {}
