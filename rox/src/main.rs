use rox::rox::rox::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Rox::new();
    if args.len() > 1 {
        println!("Usage: rox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        lox.run_file(&args[0])
    } else {
        lox.run_prompt();
    }
}
