use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::scanner::Scanner;

pub fn run_file(filename: &String) -> () {
    // get path
    // read file as bytes
    // run the string
    let path = Path::new(filename);
    let mut buf = Vec::new();
    let _ = File::open(path)
        .expect("file not found")
        .read_to_end(&mut buf);
    run(&String::from_utf8_lossy(&buf).into());
}

pub fn run_prompt() {
    let stdin = std::io::stdin();

    loop {
        print!("> ");
        let mut input = String::new();
        let _ = stdin.read_line(&mut input);

        if input.len() == 0 {
            break;
        }

        run(&input);
    }
}

fn run(contents: &String) -> () {
    let scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{token}");
    }
}

pub struct Lox {
    has_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { has_error: false }
    }
}
