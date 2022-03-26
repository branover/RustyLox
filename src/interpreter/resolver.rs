use std::collections::HashMap;

use super::{
    Stmt,
    Expr,
    Token,
};

type ResolveResult<T> = Result<T, ResolvingError>;

#[derive(Debug)]
pub enum ResolvingError {
    ReferencedInInitializer(Token, String),
    AlreadyExists(Token, String),
    ReturnOutOfFunc(Token, String)
}

impl std::fmt::Display for ResolvingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ResolvingError::ReferencedInInitializer(ref token, ref message) => {
                write!(f, "[line {}] LocalReferencedInInitializer: {}: {}",
                token.line,
                message,
                token.lexeme)
            }
            ResolvingError::AlreadyExists(ref token, ref message) => {
                write!(f, "[line {}] AlreadyExists: {}: {}",
                token.line,
                message,
                token.lexeme)
            },
            ResolvingError::ReturnOutOfFunc(ref token, ref message) => {
                write!(f, "[line {}] ReturnOutOfFunc: {}: {}",
                token.line,
                message,
                token.lexeme)
            }
        }
    }
}

impl std::error::Error for ResolvingError {
    fn description(&self) -> &str {
        match *self {
            ResolvingError::ReferencedInInitializer(_, _) => "ReferencedInInitializer",
            ResolvingError::AlreadyExists(_, _) => "AlreadyExists",
            ResolvingError::ReturnOutOfFunc(_, _) => "ReturnOutOfFunc",
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum FuncType {
    None,
    Function,
}

pub struct Resolver{
    scopes: Vec<HashMap<String, bool>>,
    current_function: FuncType,
}

impl Resolver {
    pub fn new() -> Resolver { 
        Resolver { 
            scopes: Vec::new(),
            current_function: FuncType::None
        }
    }

    pub fn resolve_ast(&mut self, ast: &mut [Stmt]) -> ResolveResult<()> {
        for stmt in ast.iter_mut() {
            self.resolve_stmt(stmt)?;
        };
        Ok(())
    }


    fn resolve_stmt(&mut self, stmt: &mut Stmt) -> ResolveResult<()> {
        match stmt {
            Stmt::ExprStmt(expr) => {
                self.resolve_expr(expr)?;
            },
            Stmt::PrintStmt(expr) => {
                self.resolve_expr(expr)?;
            },
            Stmt::VarDecl(name, initializer) => {
                self.declare(&name)?;
                if let Some(initializer) = initializer {
                    self.resolve_expr(initializer)?;
                }
                self.define(&name);
            },
            Stmt::Block(stmts) => {
                self.begin_scope();
                self.resolve_ast(stmts)?;
                self.end_scope();
            },
            Stmt::If(condition, then, else_stmt) => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then)?;
                if let Some(else_stmt) = else_stmt {
                    self.resolve_stmt(else_stmt)?;
                }
            },
            Stmt::While(condition, body) => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
            },
            Stmt::Function(name, parameters, body) => {
                self.declare(&name)?;
                self.define(&name);
                self.resolve_function(name, parameters, body, FuncType::Function)?;
            },
            Stmt::Return(token, expr) => {
                if self.current_function == FuncType::None {
                    return Err(ResolvingError::ReturnOutOfFunc(token.clone(), "Can't return from top-level code.".to_string()))
                }
                if let Some(expr) = expr {
                    self.resolve_expr(expr)?;
                }
            },
            Stmt::ClassDecl(name, methods) => {
                self.declare(&name)?;
                self.define(&name);
            }
        };
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &mut Expr) -> ResolveResult<()> {
        match expr {
            Expr::Literal(_) => (),
            Expr::Grouping(expr) => {
                self.resolve_expr(expr)?;
            },
            Expr::Unary(_, expr) => {
                self.resolve_expr(expr)?;
            },
            Expr::Binary(left, _, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            },
            Expr::Var(name, distance) => {
                if !self.scopes.is_empty() && self.scopes.last().unwrap().get(&name.lexeme) == Some(&false) {
                    return Err(ResolvingError::ReferencedInInitializer(
                        name.clone(),
                        "Can't read local variable in its own initializer.".to_string())
                    );
                }
                *distance = self.resolve_local(name);
            },
            Expr::Assign(name, value, distance) => {
                self.resolve_expr(value)?;
                *distance = self.resolve_local(name);
            },
            Expr::Logical(left, _, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            },
            Expr::Call(callee, _, arguments) => {
                self.resolve_expr(callee)?;
                for argument in arguments {
                    self.resolve_expr(argument)?;
                }
            },
            Expr::Get(object, name) => {
                self.resolve_expr(object)?;
            },
            Expr::Set(object, name, value) => {
                self.resolve_expr(value)?;
                self.resolve_expr(object)?;
            },
        };
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> ResolveResult<()> {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(_) = scope.insert(name.lexeme.to_string(), false) {
                return Err(ResolvingError::AlreadyExists(
                    name.clone(),
                    "Already a variable with this name in the scope".to_string()
                ))
            }
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_string(), true);
        }        
    }

    #[must_use]
    fn resolve_local(&mut self, name: &Token) -> Option<usize> {
        for (i,scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                return Some(i)
            }
        };

        None
    }

    fn resolve_function(&mut self, name: &Token, parameters: &[Token], body: &mut [Stmt], func_type: FuncType) -> ResolveResult<()> {
        let enclosing_function = self.current_function;
        self.current_function = func_type;
        
        self.begin_scope();
        for param in parameters {
            self.declare(&param)?;
            self.define(&param);
        }
        self.resolve_ast(body)?;
        self.end_scope();

        self.current_function = enclosing_function;
        Ok(())
    }



    
}