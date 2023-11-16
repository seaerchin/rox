use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn run_file(filename: &String) -> () {
    // get path
    // read file as bytes
    // run the string
    let path = Path::new(filename);
    let buf = &mut Vec::new();
    let _ = File::open(path).expect("file not found").read_to_end(buf);
    run(buf);
}

pub fn run_prompt() {
    let stdin = std::io::stdin();

    loop {
        print!("> ");
        let mut input = String::new();
        let _ = stdin.read_line(input);

        if input.len() == 0 {
            break;
        }

        run(input);
    }
}

fn run(file: &Vec<u8>) -> () {
    todo!()
}
