pub struct Reporter {
    pub has_error: bool,
}

impl Reporter {
    pub fn new() -> Reporter {
        Reporter { has_error: false }
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, whr: &str, message: &str) {
        println!("[line {line}] Error {whr}: {message}");
        self.has_error = true;
    }
}
