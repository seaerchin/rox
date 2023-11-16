use std::alloc::System;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use super::scanner::Scanner;

pub struct Lox {
    has_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { has_error: false }
    }

    pub fn run_file(&mut self, filename: &String) -> () {
        // get path
        // read file as bytes
        // run the string
        let path = Path::new(filename);
        let mut buf = Vec::new();
        let _ = File::open(path)
            .expect("file not found")
            .read_to_end(&mut buf);
        Self::run(&String::from_utf8_lossy(&buf).into());

        self.has_error = false;
    }

    pub fn run_prompt(&mut self) {
        let stdin = std::io::stdin();

        loop {
            print!("> ");
            let mut input = String::new();
            let _ = stdin.read_line(&mut input);

            if input.len() == 0 {
                break;
            }

            Self::run(&input);

            if self.has_error {
                exit(65);
            }
        }
    }

    fn run(contents: &String) {
        let scanner = Scanner::new(contents);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{token}");
        }
    }

    pub fn error(&mut self, line: isize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: isize, whr: &str, message: &str) {
        println!("[line {line}] Error {whr}: {message}");
        self.has_error = true;
    }
}
