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
            c if c.is_alphabetic()  => self.add_token(TokenType::String),
            d if d.is_numeric() => self.add_token(TokenType::Number),
            '\n' => self.line += 1,
            ' ' => (), // ignore whitespace
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
                }
                let x = self.advance();
                return Err(RloxError::InvalidInput(format!("Unexpected character '{x}' after '{c}'")).into());
            }
            e => {
                eprintln!("Unexpected character: {}", e);
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let c= self.source[self.current .. self.current + 1].chars().nth(0).unwrap();
        self.current += 1;
        c
    }

    fn advance_on_match(&mut self, expected: char) -> bool {
        if self.source[self.current .. self.current + 1].chars().nth(0).unwrap() == expected {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, "".to_string(), None, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}