use super::{Token,LoxType};
use super::{EvaluationError, EvaluationResult};

use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String,LoxType>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: &LoxType) {
        // println!("Adding var {} with val {}", name, value);
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn get(&self, token: &Token) -> EvaluationResult<&LoxType> {
        match self.values.get(&token.lexeme) {
            Some(val) => Ok(val),
            None => Err(EvaluationError::UndefinedIdentifierError(token.clone()))
        }
    }

    pub fn assign(&mut self, token: &Token, value: LoxType) -> EvaluationResult<&LoxType> {
        match self.values.get_mut(&token.lexeme) {
            Some(val) => {
                *val = value;
                Ok(val)
            }
            None => Err(EvaluationError::UndefinedIdentifierError(token.clone()))
        }
    }
}