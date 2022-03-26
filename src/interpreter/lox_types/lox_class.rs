use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::{
    Interpreter,
    LoxType,
    EvaluationError,
    Stmt,
    Environment,
    Token
};
use super::{Callable,LoxInstance};

#[derive(Debug)]
pub struct LoxClass {
    internal: Rc<LoxClassInternal>,
}

impl LoxClass {
    pub fn new(name: &str) -> LoxClass {
        LoxClass {
            internal: Rc::new(LoxClassInternal {
                name: name.to_string()
            }),               
        }
    }

    pub fn instantiate(&self) -> LoxInstance {
        LoxInstance::new(self.internal.clone())
    }
}

impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.internal)
    }
}

impl Callable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxType]) -> Result<LoxType,EvaluationError> {
        let instance = LoxInstance::new(self.internal.clone());
        
        Ok(LoxType::Instance(Rc::new(RefCell::new(instance))))
    }

    fn arity(&self) -> usize {
        0
    }
}

#[derive(Debug)]
pub struct LoxClassInternal {
    pub name: String,
}

impl std::fmt::Display for LoxClassInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

