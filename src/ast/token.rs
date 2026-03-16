use std::fmt::Display;

use strum_macros::Display;

use crate::ast::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Literal>,
}

#[derive(Debug, Clone, Display)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        literal: Option<Literal>,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match &self.literal {
            Some(l) => l.to_string(),
            None => "None".to_string(),
        };
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}
