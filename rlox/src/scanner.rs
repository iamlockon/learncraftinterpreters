use std::io::Error;

use crate::{error::RloxError, token::Token, token_type::TokenType};

#[derive(Debug)]
pub(crate) struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl <'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            ' ' | '\r' | '\t' => (), // ignore whitespace
            '\n' => self.line += 1,
            c if c == '!' || c == '=' || c == '<' || c == '>' => {
                if self.advance_on_match('=') {
                    let token_type = match c {
                        '!' => TokenType::BangEqual,
                        '=' => TokenType::EqualEqual,
                        '<' => TokenType::LessEqual,
                        '>' => TokenType::GreaterEqual,
                         x => unreachable!("Unexpected char: {x}"),
                    };
                    self.add_token(token_type);
                    return Ok(());
                }
                let x = self.advance();
                return Err(RloxError::InvalidInput(format!("Unexpected character '{x}' after '{c}'")).into());
            },
            '/' => {
                if self.advance_on_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            '"' => self.handle_string(),
            d if d.is_digit(10) => self.handle_number(),
            c if c.is_alphabetic() => self.handle_identifier(),
            e => {
                eprintln!("Unexpected character: {}", e);
            }
        }
        Ok(())
    }

    fn handle_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        if self.peek() == '.' {
            eprintln!("Invalid number format");
            return;
        }

        let literal = self.source[self.start .. self.current].parse().expect("Failed to parse number");
        self.add_token(TokenType::Number(literal));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1 .. self.current + 2].chars().nth(0).unwrap()
    }

    fn handle_identifier(&mut self) {

    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Unterminated string.");
            return;
        }

        self.advance();

        let literal = self.source[self.start + 1 .. self.current - 1].to_string();
        self.add_token(TokenType::String(literal));
    }   

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current .. self.current + 1].chars().nth(0).unwrap()
    }

    fn advance(&mut self) -> char {
        let c= self.source[self.current .. self.current + 1].chars().nth(0).unwrap();
        self.current += 1;
        c
    }

    fn advance_on_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current .. self.current + 1].chars().nth(0).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, "".to_string(), None, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}