use super::{Token,LoxType};
use super::{EvaluationError, EvaluationResult};
use super::native::funcs::Clock;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String,LoxType>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
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

    pub fn global() -> Environment {
        let mut environment = Environment::new();
        environment.fill_globals();
        environment
    }

    pub fn from(env: Rc<RefCell<Environment>>) -> Environment {
        Environment { 
            values: HashMap::new(),
            enclosing: Some(env)
        }
    }

    fn fill_globals(&mut self) {
        // Defines clock function
        self.define(
            "clock",
            &LoxType::Func(Rc::new(Clock::new()))
        );
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

    pub fn get_at(&self, token: &Token, distance: usize) -> EvaluationResult<LoxType> {
        if distance == 0 {
            return self.get(&token);
        }

        let parent = self.nth_parent(distance);

        match parent {
            Some(parent) => parent.borrow().get(&token),
            None => Err(EvaluationError::UndefinedIdentifierError(token.clone()))
        }
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

    pub fn assign_at(&mut self, token: &Token, value: LoxType, distance: usize) -> EvaluationResult<LoxType> {
        if distance == 0 {
            return self.assign(token, value);
        }

        let parent = self.nth_parent(distance);

        match parent {
            Some(parent) => parent.borrow_mut().assign(token, value),
            None =>Err(EvaluationError::UndefinedIdentifierError(token.clone()))
        }
    }

    fn nth_parent(&self, n: usize) -> Option<Rc<RefCell<Environment>>> {
        let mut parent = match self.enclosing {
            Some(ref parent) => parent.clone(),
            None => return None
        };

        for _ in 1..n {
            let env = match parent.borrow().enclosing {
                Some(ref env) => env.clone(),
                None => return None,
            };

            parent = env;
        }

        Some(parent)
    }
}