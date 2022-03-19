pub mod scanner;
pub mod token;
pub mod literal;
pub mod expr;
pub mod parser;

pub use token::Token as Token;
pub use scanner::Scanner as Scanner;
pub use literal::Literal as Literal;
pub use expr::Expr as Expr;
pub use parser::Parser as Parser;

#[derive(Debug,Clone)]
pub struct Error {
    pub message: String,
    pub line: usize,
}

pub trait ErrorReport {
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
    
    fn report(&mut self, line: usize, _where: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, _where, message);
    }    
}