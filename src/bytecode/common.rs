#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    Return = 1,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Constant,
            1 => OpCode::Return,
            _ => panic!("Unknown opcode {}", value),
        }
    }
}
