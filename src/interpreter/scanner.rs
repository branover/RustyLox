use super::token::{Token,TokenType};
use super::Literal;
use super::{Error, ErrorReport};

use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    char_ptr: CharPtr,
    error: Option<Error>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl ErrorReport for Scanner {
    fn error(&mut self, line: usize, message: &str) {
        self.error = Some(Error {
            line,
            message: message.to_string(),
        });
        self.report(line, "", message);
    }    
}

struct CharPtr {
    ptr: *const u8,
}

impl CharPtr {
    pub fn from(string: &str) -> CharPtr {
        CharPtr {
            ptr: string.as_ptr()
        }
    }

    #[inline(always)]
    pub fn advance(&mut self) -> char {
        unsafe {
            let c = *self.ptr as char;
            self.ptr = self.ptr.add(1);
            c
        }
    }

    #[inline(always)]
    pub fn offset(&self, i: isize) -> char {
        unsafe {
            *self.ptr.offset(i) as char
        }
    }
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: String::from(source),
            tokens: Vec::new(),
            char_ptr: CharPtr::from(source),
            error: None,
            start: 0,
            current: 0,
            line: 1,
            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>,Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        match &self.error {
            Some(e) => Err(e.clone()),
            None => Ok(std::mem::take(&mut self.tokens))
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_char_token(TokenType::LeftParen),
            ')' => self.add_char_token(TokenType::RightParen),
            '{' => self.add_char_token(TokenType::LeftBrace),
            '}' => self.add_char_token(TokenType::RightBrace),
            ',' => self.add_char_token(TokenType::Comma),
            '.' => self.add_char_token(TokenType::Dot),
            '-' => self.add_char_token(TokenType::Minus),
            '+' => self.add_char_token(TokenType::Plus),
            ';' => self.add_char_token(TokenType::Semicolon),
            '*' => self.add_char_token(TokenType::Star),
            '!' if self.matches('=') => self.add_char_token(TokenType::BangEqual),
            '!' => self.add_char_token(TokenType::Bang),
            '=' if self.matches('=') => self.add_char_token(TokenType::EqualEqual),
            '=' => self.add_char_token(TokenType::Equal),
            '<' if self.matches('=') => self.add_char_token(TokenType::LessEqual),
            '<' => self.add_char_token(TokenType::Less),
            '>' if self.matches('=') => self.add_char_token(TokenType::GreaterEqual),
            '>' => self.add_char_token(TokenType::Greater),
            '/' if self.matches('/') => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            },
            '/' => self.add_char_token(TokenType::Slash),
            ' ' | '\r' | '\t' => (),
            '\n' => {self.line += 1},
            '"' => self.string(), 
            '0'..='9' => self.number(), 
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),           
            _ => self.error(self.line, &format!("Unexpected character: {}",c)),
            // _ => (),
        };
    }  

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        }
        else if self.char_ptr.offset(0) != expected {
            false
        }
        else {
            self.current += 1;
            self.char_ptr.advance();
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        self.char_ptr.offset(0)
    }

    fn peek_n(&self, n: isize) -> char {
        if self.current + n as usize >= self.source.len() {
            return '\0'
        }
        self.char_ptr.offset(n)
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let literal = String::from(&self.source[self.start+1..self.current-1]);
        self.add_token(TokenType::String, Some(Literal::String(literal)));
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_n(1)) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let literal = (&self.source[self.start..self.current]).parse::<f64>().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Num(literal)));
    }

    fn identifier(&mut self) {
        while self.is_alnum(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = match self.keywords.get(text) {
            Some(t) => *t,
            None => TokenType::Identifier,
        };

        self.add_char_token(token_type);
    }

    fn is_digit(&self, c: char) -> bool {
        matches!(c, '0'..='9')
    }

    fn is_alpha(&self, c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_alnum(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn add_char_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start..self.current],
            literal,
            self.line)
        );
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.char_ptr.advance()
    }

    #[inline(always)]
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }


}
