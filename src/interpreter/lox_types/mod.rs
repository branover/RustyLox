pub mod lox_type;
pub mod callable;
pub mod lox_func;

pub use lox_type::LoxType;
pub use lox_type::LoxTypeError;
pub use callable::Callable ;
pub use lox_func::LoxFunc;

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