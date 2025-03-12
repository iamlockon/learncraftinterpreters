use std::fmt::{self, Display, Formatter};

use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    t: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(t: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Self {
            t,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.t, self.lexeme, self.literal)
    }
}