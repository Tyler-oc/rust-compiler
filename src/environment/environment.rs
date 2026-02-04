use std::collections::HashMap;

use crate::{errors::environment_error::EnvironmentError, interpreting::value::Value};
#[derive(Clone)]
pub(crate) struct Environment {
    pub(crate) values: HashMap<String, Value>,
    pub(crate) outer_environment: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(outer_environment: Option<Environment>) -> Self {
        Environment {
            values: HashMap::new(),
            outer_environment: outer_environment.map(Box::new),
        }
    }

    pub fn new_all_initialized(
        values: HashMap<String, Value>,
        outer_environment: Option<Box<Environment>>,
    ) -> Self {
        Environment {
            values: values,
            outer_environment: outer_environment,
        }
    }

    //current decision is to error when redefining a variable with var keyword
    pub fn define(&mut self, name: String, val: Value) -> Result<(), EnvironmentError> {
        match self.values.get(&name) {
            None => {
                self.values.insert(name, val);
                return Ok(());
            }
            Some(_) => Err(EnvironmentError::MultipleAssignmentVariable(name)),
        }
    }

    pub fn get(&mut self, name: String) -> Result<Value, EnvironmentError> {
        if let Some(stored_val) = self.values.get(&name) {
            return Ok(stored_val.clone());
        }
        match &mut self.outer_environment {
            Some(env) => return env.get(name),
            None => return Err(EnvironmentError::UndefinedVariable(name)),
        }
    }

    pub fn assign(&mut self, name: String, val: Value) -> Result<(), EnvironmentError> {
        if let Some(stored_val) = self.values.get_mut(&name) {
            *stored_val = val;
            return Ok(());
        }
        match &mut self.outer_environment {
            Some(env) => {
                if let Some(outer_stored_val) = env.values.get_mut(&name) {
                    *outer_stored_val = val;
                    return Ok(());
                }
            }
            None => return Err(EnvironmentError::UndefinedVariable(name)),
        }
        Err(EnvironmentError::UndefinedVariable(name))
    }
}
