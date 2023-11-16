use super::token::Token;

pub struct Scanner;

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}
