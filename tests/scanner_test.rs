use rustylox::Lox;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_punctuators() {
        let mut lox = Lox::new();
        lox.run_file("./example/scanning/punctuators.lox").unwrap();
    }

    #[test]
    fn run_strings() {
        let mut lox = Lox::new();
        lox.run_file("./example/scanning/strings.lox").unwrap();
    }

    #[test]
    fn run_numbers() {
        let mut lox = Lox::new();
        lox.run_file("./example/scanning/numbers.lox").unwrap();
    }

    #[test]
    fn run_identifiers() {
        let mut lox = Lox::new();
        lox.run_file("./example/scanning/identifiers.lox").unwrap();
    }
}