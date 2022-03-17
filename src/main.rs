
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

use rustylox::Lox;

use std::{
    env,
    error::Error,
};

mod interpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: rustylox [script]");
    } else if args.len() == 2 { 
        lox.run_file(&args[1])?;
    } else { 
        lox.run_prompt()?;
    }
    Ok(())
}
