pub mod scanner;
pub mod token;
pub mod literal;
pub mod expr;
pub mod parser;
pub mod lox_type;
pub mod stmt;
pub mod environment;

pub use token::Token as Token;
pub use scanner::Scanner as Scanner;
pub use literal::Literal as Literal;
pub use expr::Expr as Expr;
pub use parser::Parser as Parser;
pub use lox_type::LoxType as LoxType;
pub use lox_type::LoxTypeError as LoxTypeError;
pub use stmt::Stmt as Stmt;
pub use environment::Environment as Environment;

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
    UndefinedIdentifierError(Token),
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
            EvaluationError::UndefinedIdentifierError(ref token) => {
                write!(f,"[line {}] UndefinedIdentifierError with {}", token.line, token.lexeme)
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
            EvaluationError::UndefinedIdentifierError(_) => "UndefinedIdentifierError",
            EvaluationError::UnknownError => "UnknownError",
        }
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new()
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        stmts.iter().for_each(|stmt| {
            let result = self.evaluate_stmt(stmt);  
            match result {
                // Ok(result) => println!("{}", result),
                Ok(_) => (),
                Err(e) => println!("{}", e)
            };           
        });      
    }

    fn evaluate_stmt(&mut self, stmt: &Stmt) -> EvaluationResult<()> {
        match stmt {
            Stmt::PrintStmt(expr) => {println!("{}",self.evaluate_expr(expr)?);},
            Stmt::ExprStmt(expr) => {self.evaluate_expr(expr)?;},
            Stmt::VarDecl(_, _) => self.evaluate_var_stmt(stmt)?,
            // _ => unreachable!("Unhandled statement")
        }
        Ok(())
    }


    fn evaluate_expr(&mut self, expr: &Expr) -> EvaluationResult<LoxType> {
        match expr {
            Expr::Literal(literal) => self.evaluate_literal_expr(literal),
            Expr::Grouping(inner_expr) => self.evaluate_expr(inner_expr),
            Expr::Unary(token, right) => self.evaluate_unary_expr(token, right),
            Expr::Binary(left, token, right) => self.evaluate_binary_expr(left, token, right),
            Expr::Var(identifier) => self.evaluate_var_expr(identifier),
            Expr::Assign(identifier, value) => self.evaluate_assign_expr(identifier, value),
        }
    }

    fn evaluate_var_stmt(&mut self, stmt: &Stmt) -> EvaluationResult<()> {
        let mut value = LoxType::Nil;

        let (name, initializer) = match stmt {
            Stmt::VarDecl(ref name, ref initializer) => (name, initializer),
            _ => unreachable!("Unreachable") 
        };

        if let Some(initializer) = initializer {
            value = self.evaluate_expr(initializer)?;
        }

        self.environment.define(&name.lexeme, &value);
        Ok(())
    }

    fn evaluate_literal_expr(&self, literal: &Literal) -> EvaluationResult<LoxType> {
        match literal {
            Literal::Bool(val) => Ok(LoxType::Bool(*val)),
            Literal::Nil => Ok(LoxType::Nil),
            Literal::Num(ref val) => Ok(LoxType::Num(*val)),
            Literal::String(val) => Ok(LoxType::String(val.clone())),
        }
    }

    fn evaluate_unary_expr(&mut self, token: &Token, right: &Expr) -> EvaluationResult<LoxType> {
        let right = self.evaluate_expr(right)?;

        let result = match token.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => Ok(LoxType::Bool(!right.is_truthy())),
            _ => unreachable!("Unreachable")
        };

        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(EvaluationError::LoxTypeError(token.clone(), e))
        }
    }

    fn evaluate_binary_expr(&mut self, left: &Expr, token: &Token, right: &Expr) -> EvaluationResult<LoxType> {
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
            _ => unreachable!("Unreachable")
        };

        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(EvaluationError::LoxTypeError(token.clone(), e))
        }       
    }    

    fn evaluate_var_expr(&self, identifier: &Token) -> EvaluationResult<LoxType> {
        Ok(self.environment.get(identifier)?.clone())
    }

    fn evaluate_assign_expr(&mut self, identifier: &Token, value: &Expr) -> EvaluationResult<LoxType> {
        let value = self.evaluate_expr(value)?;
        Ok(self.environment.assign(identifier, value)?.clone())
    }
}


