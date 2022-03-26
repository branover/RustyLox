pub mod scanner;
pub mod token;
pub mod literal;
pub mod expr;
pub mod parser;
pub mod lox_types;
pub mod stmt;
pub mod environment;
pub mod native;
pub mod resolver;

pub use token::Token;
pub use scanner::Scanner;
pub use literal::Literal;
pub use expr::Expr;
pub use parser::Parser;
pub use lox_types::LoxType;
pub use lox_types::LoxTypeError;
pub use stmt::Stmt;
pub use environment::Environment;
pub use lox_types::{Callable, LoxFunc, LoxClass, LoxClassInternal, LoxInstance};
pub use resolver::Resolver;

use std::cell::RefCell;
use std::rc::Rc;

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
type StatementResult = Result<Option<LoxType>, EvaluationError>;

#[derive(Debug)]
pub enum EvaluationError {
    IllegalExpressionType(Expr),
    IllegalStatementType(Stmt),
    IllegalOperationError(Token),
    LoxTypeError(Token,LoxTypeError),
    UndefinedIdentifierError(Token),
    CallOnNonCallable(Token),
    WrongArity(Token,usize,usize),
    UnknownError,
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EvaluationError::IllegalExpressionType(ref expr) => {
                write!(f,"[line ?] IllegalExpressionType: {}", expr)
            },
            EvaluationError::IllegalStatementType(ref stmt) => {
                write!(f,"[line ?] IllegalStatementType: {}", stmt)
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
            EvaluationError::CallOnNonCallable(ref token) => {
                write!(f,"[line {}] CallOnNonCallable with {}", token.line, token.lexeme)
            }
            EvaluationError::WrongArity(ref token, len, arity) => {
                write!(f,"[line {}] WrongArity with {}.  Had {}, expected {}", token.line, token.lexeme, len, arity)
            }
            EvaluationError::UnknownError => write!(f,"[line ?] UnknownError"),
        }
    }
}

impl std::error::Error for EvaluationError {
    fn description(&self) -> &str {
        match *self {
            EvaluationError::IllegalExpressionType(_) => "IllegalExpressionType",
            EvaluationError::IllegalStatementType(_) => "IllegalStatementType",
            EvaluationError::IllegalOperationError(_) => "IllegalOperationError",
            EvaluationError::LoxTypeError(_,_) => "LoxTypeError",
            EvaluationError::UndefinedIdentifierError(_) => "UndefinedIdentifierError",
            EvaluationError::CallOnNonCallable(_) => "CallOnNonCallable",
            EvaluationError::WrongArity(_,_,_) => "WrongArity",
            EvaluationError::UnknownError => "UnknownError",
        }
    }
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::global()));
        
        Interpreter {
            environment: globals.clone(),
            globals: globals.clone() 
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        stmts.iter().for_each(|stmt| {
            let result = self.evaluate_stmt(stmt);  
            match result {
                // Ok(Some(result)) => println!("{}", result),
                Ok(_) => (),
                Err(e) => println!("{}", e)
            };           
        });      
    }

    fn evaluate_stmt(&mut self, stmt: &Stmt) -> StatementResult {
        match stmt {
            Stmt::PrintStmt(expr) => {
                println!("{}",self.evaluate_expr(expr)?);
                Ok(None)
            },
            Stmt::ExprStmt(expr) => {
                self.evaluate_expr(expr)?;
                Ok(None)
            },
            Stmt::VarDecl(_, _) => self.evaluate_var_stmt(stmt),
            Stmt::Block(stmts) => self.evaluate_block_stmt(stmts),
            Stmt::If(condition, then, else_stmt) =>
                self.evaluate_if_stmt(condition, then, else_stmt),
            Stmt::While(condition, body) => self.evaluate_while_stmt(condition, body),
            Stmt::Function(name, arguments, body) => 
                self.evaluate_function_stmt(name, arguments, body),
            Stmt::Return(token, value) => self.evaluate_return_stmt(token, value),
            Stmt::ClassDecl(name, methods) => self.evaluate_class_stmt(name, methods),
        }
    }

    fn evaluate_var_stmt(&mut self, stmt: &Stmt) -> StatementResult {
        let mut value = LoxType::Nil;

        let (name, initializer) = match stmt {
            Stmt::VarDecl(ref name, ref initializer) => (name, initializer),
            _ => unreachable!("Unreachable") 
        };

        if let Some(initializer) = initializer {
            value = self.evaluate_expr(initializer)?;
        }

        self.environment.borrow_mut().define(&name.lexeme, &value);
        Ok(None)
    }

    fn evaluate_block_stmt(&mut self, stmts: &[Stmt]) -> StatementResult {
        let env = Environment::from(
            self.environment.clone()
        );
        self.execute_block(stmts, env)
    }

    fn execute_block(&mut self, stmts: &[Stmt], environment: Environment) -> StatementResult {
        let environment = Rc::new(RefCell::new(environment));
        let previous = std::mem::replace(&mut self.environment, environment);

        for stmt in stmts {
            let result = self.evaluate_stmt(stmt);
            // println!("Statement: {:?}, Result: {:?}", stmt, result);
            match result {
                Err(e) => {
                    self.environment = previous;
                    return Err(e); 
                },
                Ok(Some(ret)) => {
                    self.environment = previous;
                    // println!("Returning: {}", ret);
                    return Ok(Some(ret))
                },
                _ => ()
            }
        }
        // println!("No return");
        self.environment = previous;
        Ok(None)
    }

    fn evaluate_if_stmt(&mut self, condition: &Expr, then: &Stmt, else_stmt: &Option<Box<Stmt>>) -> StatementResult {
        let ret = if self.evaluate_expr(condition)?.is_truthy() {
            self.evaluate_stmt(then)?
        } else if let Some(else_stmt) = else_stmt {
            self.evaluate_stmt(else_stmt)?
        } else {
            None
        };
        Ok(ret)
    }

    fn evaluate_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> StatementResult {
        while self.evaluate_expr(condition)?.is_truthy() {
            // println!("{:?}", body);
            if let Some(ret) = self.evaluate_stmt(body)? {
                // println!("RET: {}", ret);
                return Ok(Some(ret));
            }
        }
        Ok(None)
    }

    fn evaluate_function_stmt(&self, name: &Token, arguments: &Vec<Token>, body: &Vec<Stmt>) -> StatementResult {
        let function = LoxFunc::new(
            name.clone(),
            arguments.clone(),
            body.clone(),
            self.environment.clone()
        );
        self.environment.borrow_mut().define(&name.lexeme, &LoxType::Func(Rc::new(function)));
        Ok(None)
    }

    fn evaluate_return_stmt(&mut self, token: &Token, value: &Option<Expr>) -> StatementResult {
        let mut expr_result: Option<LoxType> = Some(LoxType::Nil);

        if let Some(value) = value {
            expr_result = Some(self.evaluate_expr(value)?);
        }

        Ok(expr_result)
    }

    fn evaluate_class_stmt(&mut self, name: &Token, methods: &[Stmt]) -> StatementResult {
        self.environment.borrow_mut().define(&name.lexeme, &LoxType::Nil);
        let class = LoxClass::new(&name.lexeme);
        self.environment.borrow_mut().assign(&name, LoxType::Class(Rc::new(class)))?;
        Ok(None)
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> EvaluationResult<LoxType> {
        match expr {
            Expr::Literal(literal) => self.evaluate_literal_expr(literal),
            Expr::Grouping(inner_expr) => self.evaluate_expr(inner_expr),
            Expr::Unary(token, right) => self.evaluate_unary_expr(token, right),
            Expr::Binary(left, token, right) => self.evaluate_binary_expr(left, token, right),
            Expr::Var(identifier, distance) => self.evaluate_var_expr(identifier, *distance),
            Expr::Assign(identifier, value, distance) => self.evaluate_assign_expr(identifier, value, *distance),
            Expr::Logical(left, token, right) => self.evaluate_logical_expr(left, token, right),
            Expr::Call(callee, paren, arguments) => self.evaluate_call_expr(callee, paren, arguments),
            Expr::Get(object, name) => self.evaluate_get_expr(object, name),
            Expr::Set(object, name, value) => self.evaluate_set_expr(object, name, value),
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

    fn evaluate_var_expr(&self, identifier: &Token, distance: Option<usize>) -> EvaluationResult<LoxType> {
        if let Some(distance) = distance {
            self.environment.borrow().get_at(identifier, distance)
        } else {
            self.globals.borrow().get(identifier)
        }
    }

    fn evaluate_assign_expr(&mut self, identifier: &Token, value: &Expr, distance: Option<usize>) -> EvaluationResult<LoxType> {
        let value = self.evaluate_expr(value)?;
        if let Some(distance) = distance {
            self.environment.borrow_mut().assign_at(identifier, value, distance)
        } else {
            self.globals.borrow_mut().assign(identifier, value)
        }
    }

    fn evaluate_logical_expr(&mut self, left: &Expr, token: &Token, right: &Expr) -> EvaluationResult<LoxType> {
        let left = self.evaluate_expr(left)?;

        match token.token_type {
            TokenType::Or if left.is_truthy() => return Ok(left),
            TokenType::And if !left.is_truthy() => return Ok(left),
            _ => ()
        };

        self.evaluate_expr(right)
    }

    fn evaluate_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> EvaluationResult<LoxType> {
        let callee = self.evaluate_expr(callee)?
            .get_callable()
            .ok_or_else(|| EvaluationError::CallOnNonCallable(paren.clone()))?;

        let mut evaluated_arguments: Vec<LoxType> = Vec::new();
        for arg in arguments {
            evaluated_arguments.push(self.evaluate_expr(arg)?);
        }
        
        if arguments.len() != callee.arity() {
            return Err(EvaluationError::WrongArity(
                paren.clone(),
                arguments.len(),
                callee.arity(),
            ));        
        }

        callee.call(self, &evaluated_arguments)
    }

    fn evaluate_get_expr(&mut self, object: &Expr, name: &Token) -> EvaluationResult<LoxType> {
        let object = self.evaluate_expr(object)?;
        if let LoxType::Instance(object) = object {
            Ok(object.borrow().get(name)?)
        } else {
            Err(EvaluationError::LoxTypeError(name.clone(), LoxTypeError::IllegalOperationError))
        }
    }

    fn evaluate_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> EvaluationResult<LoxType> {
        let object = self.evaluate_expr(object)?;
        if let LoxType::Instance(object) = object {
            let value = self.evaluate_expr(value)?;
            object.borrow_mut().set(name, &value);
            Ok(value)
        } else {
            Err(EvaluationError::LoxTypeError(name.clone(), LoxTypeError::IllegalOperationError))
        }
    }

}


