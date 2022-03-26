use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::{
    Interpreter,
    LoxType,
    EvaluationError,
    LoxFunc,
};
use super::{Callable,LoxInstance};

#[derive(Debug)]
pub struct LoxClass {
    internal: Rc<LoxClassInternal>,
}

impl LoxClass {
    pub fn new(name: &str, methods: HashMap<String, LoxFunc>, superclass: Option<Rc<LoxClass>>) -> LoxClass {
        LoxClass {
            internal: Rc::new(LoxClassInternal {
                name: name.to_string(),
                methods,
                superclass,
            }),               
        }
    }

    pub fn instantiate(&self) -> LoxInstance {
        LoxInstance::new(self.internal.clone())
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunc> {
        self.internal.find_method(name)
    }
}

impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.internal)
    }
}

impl Callable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxType]) -> Result<LoxType,EvaluationError> {
        let instance = Rc::new(
            RefCell::new(
                LoxInstance::new(self.internal.clone())
            )
        );
        
        if let Some(initializer) = self.find_method("init") {
            initializer.bind(instance.clone()).call(interpreter, arguments)?;
        }

        Ok(LoxType::Instance(instance))
    }

    fn arity(&self) -> usize {
        match self.internal.find_method("init") {
            Some(initializer) => initializer.arity(),
            None => 0,
        }
    }
}

#[derive(Debug)]
pub struct LoxClassInternal {
    pub name: String,
    pub methods: HashMap<String, LoxFunc>,
    pub superclass: Option<Rc<LoxClass>>,
}

impl std::fmt::Display for LoxClassInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxClassInternal {
    pub fn find_method(&self, name: &str) -> Option<LoxFunc> {
        if let Some(found) = self.methods.get(name).cloned() {
            return Some(found);
        }

        if let Some(ref superclass) = self.superclass {
            return superclass.find_method(name);
        }

        None
    }
}

