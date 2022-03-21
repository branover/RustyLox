use super::{Token,Expr};

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    VarDecl(Token,Option<Expr>),
    Block(Vec<Stmt>),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Stmt::ExprStmt(ref expr) => write!(f, "({})", expr),
            Stmt::PrintStmt(ref expr) => write!(f, "(print {})", expr),
            Stmt::VarDecl(ref var, _) => write!(f, "var {}", var),
            Stmt::Block(ref stmts) => write!(f, "<block of statements with len {}>", stmts.len()),
        }
    }
}