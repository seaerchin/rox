use crate::rox::token_type::TokenType;

use super::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    cur_line: i32,
}

impl Scanner {
    pub fn new(contents: &String) -> Scanner {
        Scanner {
            source: contents.to_string(),
            tokens: vec![],
            cur_line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let is_at_end = self.is_at_end();

        while !is_at_end {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            String::from(""),
            self.cur_line,
        ));

        todo!()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            _ => todo!(),
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len().try_into().unwrap();
    }

    fn advance(&mut self) -> char {
        let res = self
            .source
            .chars()
            .nth(self.current.try_into().unwrap())
            .unwrap();

        self.current += 1;

        return res;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, String::from("NULL"))
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {
        // [start, end)
        let start: usize = self.start.try_into().unwrap();
        let cur: usize = self.current.try_into().unwrap();
        let text = &self.source[start..cur];
        self.tokens.push(Token::new(
            token_type,
            String::from(text),
            literal,
            self.cur_line,
        ));
    }
}
