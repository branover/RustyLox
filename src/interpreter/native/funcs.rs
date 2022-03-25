use crate::interpreter::{
    LoxType,
    Callable,
};
use crate::interpreter::EvaluationError;

use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Debug)]
pub struct Clock {}

impl Clock {
    pub fn new() -> Clock {Clock{}}
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"<native_fn Clock>")
    }
}

impl Callable for Clock {

    fn arity(&self) -> usize {
        0
    }

    fn call(&self, interpreter: &mut crate::interpreter::Interpreter, arguments: &[LoxType]) -> Result<LoxType, EvaluationError> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(LoxType::Num(current_time as f64))
    } 
}