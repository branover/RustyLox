pub mod scanner;
pub mod token;
pub mod literal;

pub use token::Token as Token;
pub use scanner::Scanner as Scanner;
pub use literal::Literal as Literal;

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