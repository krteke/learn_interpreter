use std::fmt::{Debug, Display};

use crate::bytecode::token::{Token, TokenType};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Scan(ScanError),
    Interpret(InterpretError),
    Parser(ParserError),
    #[error(transparent)]
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Scan(e) => write!(f, "ScanError: {} at line {}", e.message, e.line),
            Error::Interpret(e) => todo!(),
            Error::Parser(e) => write!(f, "ParserError: {} at line {}", e.message, e.line),
            Error::Io(e) => write!(f, "IoError: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum InterpretError {
    Compile(String),
    Runtime(String),
}

#[derive(Debug)]
pub struct ScanError {
    pub message: String,
    pub line: usize,
}

impl ScanError {
    pub fn new(message: String, line: usize) -> Self {
        Self { message, line }
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub lexeme: String,
    pub token_type: TokenType,
    pub line: usize,
    pub message: String,
}

impl ParserError {
    pub fn new(token: &Token, message: &str) -> Self {
        Self {
            lexeme: token.lex.to_string(),
            token_type: token.token_type,
            line: token.line as usize,
            message: message.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
