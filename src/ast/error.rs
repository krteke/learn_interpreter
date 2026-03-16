use crate::ast::{token::Token, token_type::TokenType};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Scan(ScanError),
    Parser(ParserError),
    Runtime(RuntimeError),
    #[error(transparent)]
    Io(std::io::Error),
}

#[derive(Debug)]
pub struct ScanError {
    line: usize,
    message: String,
}

#[derive(Debug)]
pub struct ParserError {
    token: Token,
    message: String,
}

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl ParserError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl ScanError {
    pub fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Scan(err) => write!(f, "[line {}] error: {}", err.line, err.message),
            Error::Parser(err) => {
                let at = if err.token.token_type == TokenType::EOF {
                    " at end".to_string()
                } else {
                    format!(" at '{}'", err.token.lexeme)
                };

                write!(f, "{}{}: {}", err.token.line, at, err.message)
            }
            Error::Runtime(err) => {
                write!(
                    f,
                    "{} at '{}': {}",
                    err.token.line, err.token.lexeme, err.message
                )
            }
            Error::Io(err) => write!(f, "io error: {}", err),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
