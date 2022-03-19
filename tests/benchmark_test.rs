#![feature(test)]

use rustylox::Lox;
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::fs;

    #[bench]
    fn bench_directory(b: &mut Bencher) {
        b.iter(|| {
            let paths = fs::read_dir("./example/benchmark").unwrap();
            paths.for_each(|path| {
                let path = path.unwrap().path();
                let path = path.to_str().unwrap();
                println!("\nRunning file: {}",path);
                let mut lox = Lox::new();
                lox.run_file(path).unwrap();                
            });
            
        })
    }
}