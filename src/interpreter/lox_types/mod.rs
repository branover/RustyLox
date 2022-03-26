use super::{Interpreter,EvaluationError};

pub mod lox_type;
pub mod lox_func;
pub mod lox_class;
pub mod lox_instance;

pub use lox_type::LoxType;
pub use lox_type::LoxTypeError;
pub use lox_func::LoxFunc;
pub use lox_class::{LoxClass,LoxClassInternal};
pub use lox_instance::LoxInstance;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum FuncType {
    Function,
    Method,
}

impl std::fmt::Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FuncType::Function => write!(f, "function"),
            FuncType::Method => write!(f, "method")
        }
    }
}

pub trait Callable: std::fmt::Debug + std::fmt::Display {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxType]) -> Result<LoxType, EvaluationError>;
    fn arity(&self) -> usize;
}