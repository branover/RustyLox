#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

mod interpreter;

use interpreter::*;

use std::{
    fs::read_to_string,
    io,
};

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    interpreter: Interpreter,
}    

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorReport for Lox {}

impl Lox {
    pub fn new() -> Lox {
        Lox{
            had_error: false,
            had_runtime_error: false,
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, path: &str) -> std::io::Result<()> {
        let contents = read_to_string(path)?;
        self.run(&contents);

        if self.had_error {
            std::process::exit(65);
        }
        if self.had_runtime_error {
            std::process::exit(70);
        }
        Ok(())
    }
    
    pub fn run_prompt(&mut self) -> std::io::Result<()> { 
        let stdin = io::stdin();
        loop {
            let mut buffer = String::new();
            let read_len = stdin.read_line(&mut buffer)?;
            if read_len == 0 {
                break;
            }
            self.run(&buffer);
            self.had_error = false;
        }
        Ok(())
    }
    
    fn run(&mut self, source: &str) {
        // Scanning
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();
        if let Ok(ref tokens) = result {
            tokens.iter().for_each(|token| {
                // println!("{}", token);
            });
        } 
        else if let Err(ref e) = result {
            self.had_error = true;
            return;
        }
        
        // Parsing 
        let tokens = result.unwrap();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();
        match stmts {
            Ok(stmts) => self.interpreter.interpret(stmts),
            Err(e) => {
                println!("{}", e);
                self.had_runtime_error = true;
            }
        };
    }
}
