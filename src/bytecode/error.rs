use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum VMError {
    Scan(ScanError),
    Interpret(InterpretError),
    #[error(transparent)]
    Io(std::io::Error),
}

impl Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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

pub type Result<T> = std::result::Result<T, VMError>;
