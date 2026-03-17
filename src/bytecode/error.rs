pub enum VMError {
    Interpret(InterpretError),
}

pub enum InterpretError {
    Compile(String),
    Runtime(String),
}

pub type Result<T> = std::result::Result<T, VMError>;
