use std::cell::RefCell;
use std::rc::Rc;

use super::super::{
    Interpreter,
    LoxType,
    EvaluationError,
    Stmt,
    Environment,
    Token
};
use super::Callable;

#[derive(Debug)]
pub struct LoxFunc {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunc {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>, closure: Rc<RefCell<Environment>>) -> LoxFunc {
        LoxFunc {
            name,
            params,
            body,
            closure,                
        }
    }
}

impl std::fmt::Display for LoxFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}

impl Callable for LoxFunc {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxType]) -> Result<LoxType,EvaluationError> {
        let mut environment = Environment::from(self.closure.clone());

        for (i,param) in self.params.iter().enumerate() {
            let arg = arguments.get(i).unwrap();
            environment.define(&param.lexeme, arg);
        }
        match interpreter.execute_block(&self.body, environment)? {
            Some(ret) => {
                // println!("RETURN :{}", ret);
                Ok(ret)
            },
            None => {
                // println!("NORETURN");
                Ok(LoxType::Nil)
            }
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
