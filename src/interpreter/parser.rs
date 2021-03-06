use super::token::{Token,TokenType};
use super::Literal;
use super::Expr;
use super::Stmt;
use super::lox_types::FuncType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type ParseResult<T> = Result<T, ParsingError>;

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedTokenError(Token, String),
    UnexpectedEofError,  
    InvalidAssignmentError(Token),
    TooManyArgumentsError,
    TooManyParametersError,
    InternalError(String)  
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParsingError::UnexpectedTokenError(ref token, ref message) => {
                write!(f,
                       "[line {}] UnexpectedTokenError: {} {}",
                       token.line,
                       message,
                       token.lexeme)
            }
            ParsingError::UnexpectedEofError => f.write_str("Unexpected end of input"),
            ParsingError::InvalidAssignmentError(ref token) => {
                write!(f, "[line {}] Invalid assignment target", token.line)
            }
            ParsingError::InternalError(ref message) => write!(f, "Internal error: {}", message),
            ParsingError::TooManyArgumentsError => f.write_str("Too many arguments, max number is 255"),
            ParsingError::TooManyParametersError => f.write_str("Too many parameters, max number is 255")
        }
    }
}

impl std::error::Error for ParsingError {
    fn description(&self) -> &str {
        match *self {
            ParsingError::UnexpectedTokenError(_, _) => "UnexpectedTokenError",
            ParsingError::UnexpectedEofError => "UnexpectedEofError",
            ParsingError::InvalidAssignmentError(_) => "InvalidAssignmentError",
            ParsingError::InternalError(_) => "InternalError",
            ParsingError::TooManyArgumentsError => "TooManyArgumentsError",
            ParsingError::TooManyParametersError => "TooManyParametersError"
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> ParseResult<Stmt> {
        let peek = self.peek();
        let result = match peek.token_type {
            TokenType::Var => {
                self.advance();
                self.var_declaration()
            },
            TokenType::Class => {
                self.advance();
                self.class_declaration()
            }
            TokenType::Fun => {
                self.advance();
                self.function(FuncType::Function)
            }
            _ => self.statement()
        };

        match result {
            Ok(stmt) => Ok(stmt),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?.clone();
        let mut initializer = None;

        if self.match_token(&[TokenType::Equal]) {
            let expr = self.expression()?;
            initializer = Some(expr);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::VarDecl(name, initializer))
    }

    fn class_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?.clone();
        
        let mut superclass = None;
        if self.match_token(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            superclass = Some(Expr::Var(self.previous().clone(), None));
        }

        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function(FuncType::Method)?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;
        Ok(Stmt::ClassDecl(name, methods, superclass))
    }

    
    fn function(&mut self, func_type: FuncType) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", func_type))?.clone();
        self.consume(TokenType::LeftParen, &format!("Expect '(' after {} name.", func_type))?;

        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(ParsingError::TooManyArgumentsError)
                }
                let param = self.consume(TokenType::Identifier, "Expect parameter name.")?.clone();
                parameters.push(param);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }        
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(TokenType::LeftBrace, &format!("Expect '{{' before {} body.", func_type))?;
        let body = self.block_statement()?;
        Ok(Stmt::Function(name.clone(), parameters, body))
    }


    fn statement(&mut self) -> ParseResult<Stmt> {
        let peek = self.peek();
        match peek.token_type {
            TokenType::For => {
                self.advance();
                self.for_statement()
            }
            TokenType::If => {
                self.advance();
                self.if_statement()
            }
            TokenType::Print => {
                self.advance();
                self.print_statement()
            },
            TokenType::Return => {
                self.advance();
                self.return_statement()
            },
            TokenType::While => {
                self.advance();
                self.while_statement()
            }
            TokenType::LeftBrace => {
                self.advance();
                Ok(Stmt::Block(self.block_statement()?))
            }
            _ => self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut initializer: Option<Stmt> = None;
        if self.match_token(&[TokenType::Semicolon]) {}
        else if self.match_token(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Expr> = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment: Option<Expr> = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }

        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;    
        
        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(vec![
                body,
                Stmt::ExprStmt(increment)
            ]);
        }

        body = Stmt::While(
            condition.unwrap_or(Expr::Literal(Literal::Bool(true))),
            Box::new(body)
        );

        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![
                initializer,
                body
            ]);
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.match_token(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If(condition, Box::new(then_branch), else_branch))
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::PrintStmt(value))
    }

    fn return_statement(&mut self) -> ParseResult<Stmt> {
        let keyword = self.previous().clone();
        let mut value: Option<Expr> = None;

        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(keyword, value))
    }

    fn while_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let body = self.statement()?;
        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn block_statement(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::ExprStmt(expr))
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.or()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Var(token,_) = expr {
                return Ok(Expr::Assign(token, Box::new(value), None))
            } else if let Expr::Get(object, name) = expr {
                return Ok(Expr::Set(object, name, Box::new(value)))
            } else {
                return Err(ParsingError::InvalidAssignmentError(equals))
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)        
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        let matches = vec![
            TokenType::BangEqual,
            TokenType::EqualEqual
        ];

        while self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;

        let matches = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ];

        while self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        let matches = vec![
            TokenType::Minus,
            TokenType::Plus,
        ];

        while self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)        
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        let matches = vec![
            TokenType::Slash,
            TokenType::Star,
        ];

        while self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)        
    }    

    fn unary(&mut self) -> ParseResult<Expr> {
        let matches = vec![
            TokenType::Bang,
            TokenType::Minus
        ];

        let expr = if self.match_token(&matches) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Expr::Unary(operator, Box::new(right))          
        } else {
            self.call()?
        };
        Ok(expr)
    }

    fn call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&[TokenType::Dot]) {
                let name = self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get(Box::new(expr), name.clone());
            } else {
                break;
            }

        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> ParseResult<Expr> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParsingError::TooManyArgumentsError)
                }
                arguments.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expect ')' afte rarguments."
        )?;

        Ok(Expr::Call(Box::new(callee), paren.clone(), arguments))
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        if self.match_token(&[
            TokenType::Number,
            TokenType::String,
            TokenType::False,
            TokenType::True,
            TokenType::Nil,        
        ]) {
            return match self.previous().literal {
                Some(ref literal) => Ok(Expr::Literal(literal.clone())),
                None => Err(ParsingError::InternalError(
                    "Undefined Literal".to_string()
                ))
            }
        }

        if self.match_token(&[TokenType::Super]) {
            let keyword = self.previous().clone();
            self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
            let method = self.consume(
                TokenType::Identifier,
                "Expect superclass method name."
            )?.clone();
            return Ok(Expr::Super(keyword, method, None));
        }

        if self.match_token(&[TokenType::This]) {
            return Ok(Expr::This(self.previous().clone(), None));
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Var(self.previous().clone(), None));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::from(expr)))
        } 

        if self.is_at_end() {
            Err(ParsingError::UnexpectedEofError)
        } else {
            Err(ParsingError::UnexpectedTokenError(self.peek().clone(), "Unexpected Token".to_string()))
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

    fn consume(&mut self, token_type: TokenType, msg: &str) -> ParseResult<&Token> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let peek = self.peek().clone();
            Err(ParsingError::UnexpectedTokenError(
                self.peek().clone(),
                msg.to_string(),
            ))
        }
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
