#![feature(test)]

use rustylox::Lox;
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_scanning(b: &mut Bencher) {
        b.iter(|| {
            let mut lox = Lox::new();
            lox.run_file("/etc/passwd").unwrap();            
        })
    }
}