#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    ConstantLong = 1,
    Return = 2,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Constant,
            1 => OpCode::ConstantLong,
            2 => OpCode::Return,
            _ => panic!("Unknown opcode {}", value),
        }
    }
}
