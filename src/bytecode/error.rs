use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum VMError {
    Interpret(InterpretError),
}

// impl std::error::Error for VMError {
//     fn cause(&self) -> Option<&dyn std::error::Error> {
//         todo!()
//     }
// }

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

pub type Result<T> = std::result::Result<T, VMError>;
