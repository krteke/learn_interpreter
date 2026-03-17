#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    ConstantLong = 1,
    Negate = 2,
    Return = 3,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Constant,
            1 => OpCode::ConstantLong,
            2 => OpCode::Negate,
            3 => OpCode::Return,
            _ => panic!("Unknown opcode {}", value),
        }
    }
}
