use super::{Token,Expr};

#[derive(Debug,Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    VarDecl(Token,Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr,Box<Stmt>,Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Vec<Stmt>),
    Return(Token, Option<Expr>),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Stmt::ExprStmt(ref expr) => write!(f, "({})", expr),
            Stmt::PrintStmt(ref expr) => write!(f, "(print {})", expr),
            Stmt::VarDecl(ref var, _) => write!(f, "var {}", var),
            Stmt::Block(ref stmts) => write!(f, "<block of statements with len {}>", stmts.len()),
            Stmt::If(ref expr, ref stmt, ref else_stmt) => {
                write!(f, "if ({}) then ({}) else {:?}", expr, stmt, else_stmt)
            },
            Stmt::While(ref expr, ref stmt) => write!(f, "while ({}): {}", expr, stmt),
            Stmt::Function(ref name, _,_) => write!(f, "<function {}>", name),
            Stmt::Return(_, ref expr) => write!(f, "return {:?}", expr),
        }
    }
}