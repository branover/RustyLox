// pub type Object = String;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Num(f64),
}