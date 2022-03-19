// pub type Object = String;
use std; 

#[derive(Debug,PartialEq,Clone)]
pub enum Literal {
    String(String),
    Num(f64),
    Bool(bool),
    Nil
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Literal::String(ref string) => write!(f, "{}", string),
            Literal::Num(ref number) => write!(f, "{}", number),
            Literal::Bool(ref b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}