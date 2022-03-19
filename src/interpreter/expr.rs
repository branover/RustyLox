use super::Literal;
use super::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Expr::Binary(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(ref expr) => write!(f, "(group {})", expr),
            Expr::Literal(ref literal) => write!(f, "{}", literal),
            Expr::Unary(ref operator, ref expr) => write!(f, "({} {})", operator.lexeme, expr),
            // Expr::Var(ref token, _) => write!(f, "(var {})", token.lexeme),
            // Expr::Assign(ref token, ref expr, _) => write!(f, "(assign {} {})", token.lexeme, expr),
            // Expr::Logical(ref left, ref operator, ref right) => {
            //     write!(f, "({} {} {})", operator.lexeme, left, right)
            // }
            // Expr::Call(ref callee, ref arguments, _) => {
            //     write!(f, "(call {} {:?})", callee, arguments)
            // }
            // Expr::Get(ref expr, ref token) => write!(f, "(get {} {})", token.lexeme, expr),
            // Expr::Set(ref expr, ref token, _) => write!(f, "(set {} {})", token.lexeme, expr),
            // Expr::This(_, _) => write!(f, "this"),
            // Expr::Super(_, ref method, _) => write!(f, "(super {})", method.lexeme),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::token::TokenType;

    #[test]
    fn print_test() {
        let expr = Expr::Binary(
            Box::from(Expr::Unary(
                Token::new(TokenType::Minus, "-", None, 1),
                Box::from(Expr::Literal(Literal::Num(123.0)))
            )),
            Token::new(TokenType::Star, "*", None, 1),
            Box::from(Expr::Grouping(
                Box::from(Expr::Literal(Literal::Num(45.67)))
            ))
        );

        println!("{}",expr);
    } 
}
