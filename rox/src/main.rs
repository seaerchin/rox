use rox::rox::rox::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: rox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0])
    } else {
        run_prompt();
    }
}
