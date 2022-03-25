use super::super::{
    Interpreter,
    LoxType,
    EvaluationError
};

pub trait Callable: std::fmt::Debug + std::fmt::Display {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxType]) -> Result<LoxType, EvaluationError>;
    fn arity(&self) -> usize;
}