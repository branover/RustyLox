use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::{
    LoxType,
    EvaluationError,
    Token,
};
use super::{LoxClassInternal};

#[derive(Debug, Clone)]
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
        if let Some(val) = self.fields.get(&name.lexeme).cloned() {
            return Ok(val);
        }

        if let Some(method) = self.class.find_method(&name.lexeme) {
            let method = method.bind(Rc::new(RefCell::new(self.clone())));
            return Ok(LoxType::Func(Rc::new(method)));
        }
        
        Err(EvaluationError::UndefinedIdentifierError(name.clone()))
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