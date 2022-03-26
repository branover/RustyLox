use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::{
    Interpreter,
    LoxType,
    EvaluationError,
    Stmt,
    Environment,
    Token,
};
use super::{Callable,LoxClassInternal};

#[derive(Debug)]
pub struct LoxInstance {
    class: Rc<LoxClassInternal>,
    fields: HashMap<String, LoxType>,
}

impl LoxInstance {
    pub fn new(class: Rc<LoxClassInternal>) -> LoxInstance {
        LoxInstance {
            class,  
            fields: HashMap::new(),             
        }
    }

    pub fn get(&self, name: &Token) -> Result<LoxType, EvaluationError> {
        match self.fields.get(&name.lexeme) {
            Some(val) => Ok(val.clone()),
            None => Err(EvaluationError::UndefinedIdentifierError(name.clone()))
        }
    }

    pub fn set(&mut self, name: &Token, value: &LoxType) {
        self.fields.insert(name.lexeme.clone(), value.clone());
    }
}

impl std::fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.class)
    }
}

// impl Callable for LoxInstance {
//     fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxType]) -> Result<LoxType,EvaluationError> {
//         let instance = LoxInstance(self);
        
//     }

//     fn arity(&self) -> usize {
//         0
//     }
// }