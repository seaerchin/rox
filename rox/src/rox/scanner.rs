use super::token::Token;

pub struct Scanner;

impl Scanner {
    pub fn new(contents: &String) -> Scanner {
        Scanner {}
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}
