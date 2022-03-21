use super::{Token,LoxType};
use super::{EvaluationError, EvaluationResult};

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String,LoxType>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from(env: Rc<RefCell<Environment>>) -> Environment {
        Environment { 
            values: HashMap::new(),
            enclosing: Some(env)
        }
    }

    pub fn define(&mut self, name: &str, value: &LoxType) {
        // println!("Adding var {} with val {}", name, value);
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn get(&self, token: &Token) -> EvaluationResult<LoxType> {
        if let Some(val) = self.values.get(&token.lexeme) {
            return Ok(val.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(token)
        }

        Err(EvaluationError::UndefinedIdentifierError(token.clone()))
    }

    pub fn assign(&mut self, token: &Token, value: LoxType) -> EvaluationResult<LoxType> {
        if let Some(val) = self.values.get_mut(&token.lexeme) {
            *val = value;
            return Ok(val.clone());
        };

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(token, value);
        }

        Err(EvaluationError::UndefinedIdentifierError(token.clone()))
    }
}