use std::cell::RefCell;
use std::rc::Rc;

use super::super::{
    Interpreter,
    LoxType,
    EvaluationError,
    Stmt,
    Environment,
    Token,
    LoxInstance,
    TokenType
};
use super::Callable;

#[derive(Debug,Clone)]
pub struct LoxFunc {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl LoxFunc {
    pub fn new(name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool
    ) -> LoxFunc {
        LoxFunc {
            name,
            params,
            body,
            closure, 
            is_initializer,               
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunc {
        let mut environment = Environment::from(self.closure.clone());
        environment.define("this", &LoxType::Instance(instance.clone()));
        LoxFunc::new(
            self.name.clone(),
            self.params.clone(),
            self.body.clone(),
            Rc::new(RefCell::new(environment)),
            self.is_initializer
        )
    }
}

impl std::fmt::Display for LoxFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}

impl Callable for LoxFunc {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxType]) -> Result<LoxType,EvaluationError> {
        // if self.is_initializer {
        //     return self.closure.borrow().get_at(
        //         &Token::new(TokenType::This, "this", None, 0),
        //         0
        //     );
        // }
        
        let mut environment = Environment::from(self.closure.clone());

        for (i,param) in self.params.iter().enumerate() {
            let arg = arguments.get(i).unwrap();
            environment.define(&param.lexeme, arg);
        }

        match interpreter.execute_block(&self.body, environment)? {
            _ if self.is_initializer => self.closure.borrow().get_at(
                &Token::new(TokenType::This, "this", None, 0),
                0
            ),
            Some(ret) => Ok(ret),
            None => Ok(LoxType::Nil)
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
