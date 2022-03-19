use super::token::{Token,TokenType};
use super::Literal;
use super::{Error, ErrorReport};
use super::{Expr};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    error: Option<Error>
}

impl ErrorReport for Parser {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            error: None
        }
    }

    pub fn parse(&mut self) -> Result<Expr, Error> {
        let expr = self.expression();

        match self.error {
            None => {
                Ok(expr)
            },
            Some(ref e) => Err(e.clone())
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        let matches = vec![
            TokenType::BangEqual,
            TokenType::EqualEqual
        ];

        while self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        let matches = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ];

        while self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        let matches = vec![
            TokenType::Minus,
            TokenType::Plus,
        ];

        while self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        expr        
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        let matches = vec![
            TokenType::Slash,
            TokenType::Star,
        ];

        while self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        expr        
    }    

    fn unary(&mut self) -> Expr {
        let matches = vec![
            TokenType::Bang,
            TokenType::Minus
        ];

        if self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.unary();
            let expr = Expr::Unary(operator, Box::new(right));
            expr           
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&[TokenType::False]) {
            Expr::Literal(Literal::Bool(false))
        } else if self.match_token(&[TokenType::True]) {
            Expr::Literal(Literal::Bool(true))
        } else if self.match_token(&[TokenType::Nil]) {
            Expr::Literal(Literal::Nil)
        } else if self.match_token(&[TokenType::Number, TokenType::String]) {
            Expr::Literal(self.previous().literal.as_ref().unwrap().clone())
        } else if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            Expr::Grouping(Box::from(expr))
        } else {
            println!("Trying to parse Token: {}", self.peek());
            self.parse_error(&self.peek().clone(), "Expected expression");
        }
    }

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        let matched = token_types
            .iter()
            .any(|token_type| self.check(*token_type));
        
        if matched {
            self.advance();
        }
        matched
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current-1]
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> &Token {
        if self.check(token_type) {
            self.advance()
        } else {
            let peek = self.peek().clone();
            self.parse_error(&peek, msg)
        }
    }

    fn parse_error(&mut self, token: &Token, msg: &str) -> ! {
        self.error = Some(Error {
            line: token.line,
            message: msg.to_string(),
        });
        match token.token_type {
            TokenType::Eof => {
                self.report(token.line, " at end", msg)
            }
            _ => {
                self.report(token.line, &format!(" at '{}'",token.lexeme), msg);
            }
        }
        panic!("Parsing error! Reason: {}", msg);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | 
                TokenType::For | TokenType::If | TokenType::While | 
                TokenType::Print | TokenType::Return => return,
                _ => ()
            };

            self.advance();
        }
    }



}
