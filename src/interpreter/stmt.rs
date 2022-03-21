use super::{Token,Expr};

pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    VarDecl(Token,Option<Expr>),
}