use crate::rox::token_type::TokenType;

use super::substring::Substring;
use super::{error::Reporter, token::Token};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    cur_line: i32,
    reporter: Reporter,
}

impl Scanner {
    pub fn new(contents: &String) -> Scanner {
        Scanner {
            source: contents.to_string(),
            tokens: vec![],
            cur_line: 1,
            start: 0,
            current: 0,
            reporter: Reporter::new(),
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
            '!' => {
                let is_match = self.match_next('=');
                self.add_token(if is_match {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                })
            }
            '=' => {
                let is_match = self.match_next('=');
                self.add_token(if is_match {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                })
            }
            '<' => {
                let is_match = self.match_next('=');
                self.add_token(if is_match {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                })
            }
            '>' => {
                let is_match = self.match_next('=');
                self.add_token(if is_match {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                })
            }
            '/' => {
                // NOTE: This is a comment - we parse until EOL
                if self.match_next('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.cur_line += 1,
            '"' => self.string(),
            _ => {
                if is_digit(c) {
                    self.number()
                }
                self.reporter.error(self.cur_line, "Unexpected character")
            }
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len().try_into().unwrap();
    }

    // NOTE: This consumes the current token
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
        let text = self.source.substring(start, cur);
        self.tokens.push(Token::new(
            token_type,
            String::from(text),
            literal,
            self.cur_line,
        ));
    }

    fn match_next(&mut self, expected: char) -> bool {
        match self
            .source
            .chars()
            // verified above that it's not beyond the end
            .nth(self.current.try_into().unwrap())
        {
            Some(c) => {
                if c != expected {
                    return false;
                }
                self.current += 1;
                return true;
            }
            None => return false,
        }
    }

    fn peek(&self) -> Option<char> {
        // NOTE: Look at next token in stream without consuming
        self.source.chars().nth(self.current.try_into().unwrap())
    }

    fn string(&mut self) -> () {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.cur_line += 1;
            }
            self.advance();
        }

        // NOTE: Check if we reached EOF or terminating `"`
        if self.is_at_end() {
            self.reporter.error(self.cur_line, "Unterminated string");
        }

        // NOTE: Consume the closing `"`
        self.advance();

        // NOTE: Add consumed token to list
        let substring = self
            .source
            .substring(self.start as usize + 1, self.current as usize - 1);
        self.add_token_with_literal(TokenType::STRING, String::from(substring))
    }

    fn number(&mut self) {
        loop {
            match self.peek() {
                Some(c) => {
                    if is_digit(c) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                // NOTE: The don't have to handle `None` case
                // as this implies we hit a number prior to EOF
                None => todo!(),
            }
        }
    }
}

fn is_digit(c: char) -> bool {
    if c >= '0' && c <= '9' {
        return true;
    }
    return false;
}
