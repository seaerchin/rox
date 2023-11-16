pub fn error(line: isize, message: &str) {
    report(line, "", message);
}

fn report(line: isize, whr: &str, message: &str) {
    println!("[line {line}] Error {whr}: {message}");
    let had_error = true;
}
