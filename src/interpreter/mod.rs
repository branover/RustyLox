pub mod scanner;
pub mod token;
pub mod literal;
pub mod expr;
pub mod parser;
pub mod lox_type;

pub use token::Token as Token;
pub use scanner::Scanner as Scanner;
pub use literal::Literal as Literal;
pub use expr::Expr as Expr;
pub use parser::Parser as Parser;
pub use lox_type::LoxType as LoxType;
pub use lox_type::LoxTypeError as LoxTypeError;

use token::TokenType;

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

type EvaluationResult<T> = Result<T, EvaluationError>;

#[derive(Debug)]
pub enum EvaluationError {
    IllegalExpressionType(Expr),
    IllegalOperationError(Token),
    LoxTypeError(Token,LoxTypeError),
    UnknownError,
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EvaluationError::IllegalExpressionType(ref expr) => {
                write!(f,"[line ?] IllegalExpressionType: {}", expr)
            },
            EvaluationError::IllegalOperationError(ref token) => {
                write!(f,"[line {}] IllegalOperationError: {}", token.line, token.lexeme)
            },
            EvaluationError::LoxTypeError(ref token, ref e) => {
                write!(f,"[line {}] LoxTypeError with {}: {}", token.line, token.lexeme, e)
            },
            EvaluationError::UnknownError => write!(f,"[line ?] UnknownError"),
        }
    }
}

impl std::error::Error for EvaluationError {
    fn description(&self) -> &str {
        match *self {
            EvaluationError::IllegalExpressionType(_) => "IllegalExpressionType",
            EvaluationError::IllegalOperationError(_) => "IllegalOperationError",
            EvaluationError::LoxTypeError(_,_) => "LoxTypeError",
            EvaluationError::UnknownError => "UnknownError",
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, expr: Expr) {
        // println!("{}", expr);
        let result = self.evaluate_expr(&expr);
        match result {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e)
        };      
    }


    fn evaluate_expr(&self, expr: &Expr) -> EvaluationResult<LoxType> {
        match expr {
            Expr::Literal(literal) => self.evaluate_literal_expr(literal),
            Expr::Grouping(inner_expr) => self.evaluate_expr(inner_expr),
            Expr::Unary(token, right) => self.evaluate_unary_expr(token, right),
            Expr::Binary(left, token, right) => self.evaluate_binary_expr(left, token, right),
        }
    }

    fn evaluate_literal_expr(&self, literal: &Literal) -> EvaluationResult<LoxType> {
        match literal {
            Literal::Bool(val) => Ok(LoxType::Bool(*val)),
            Literal::Nil => Ok(LoxType::Nil),
            Literal::Num(ref val) => Ok(LoxType::Num(*val)),
            Literal::String(val) => Ok(LoxType::String(val.clone())),
        }
    }

    fn evaluate_unary_expr(&self, token: &Token, right: &Expr) -> EvaluationResult<LoxType> {
        let right = self.evaluate_expr(right)?;

        let result = match token.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => Ok(LoxType::Bool(!right.is_truthy())),
            _ => panic!("Unreachable")
        };

        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(EvaluationError::LoxTypeError(token.clone(), e))
        }
    }

    fn evaluate_binary_expr(&self, left: &Expr, token: &Token, right: &Expr) -> EvaluationResult<LoxType> {
        let left = self.evaluate_expr(left)?;
        let right = self.evaluate_expr(right)?;

        let result = match token.token_type {
            TokenType::Minus => left - right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::Plus => left + right,
            TokenType::Greater |
            TokenType::GreaterEqual |
            TokenType::Less|
            TokenType::LessEqual |
            TokenType::BangEqual |
            TokenType::EqualEqual => left.determine_ordering(&right, token.token_type),
            _ => panic!("Unreachable")
        };

        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(EvaluationError::LoxTypeError(token.clone(), e))
        }       
    }    
}


