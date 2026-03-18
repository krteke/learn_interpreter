#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    ConstantLong = 1,
    Add = 2,
    Subtract = 3,
    Multiply = 4,
    Divide = 5,
    Negate = 6,
    Return = 7,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Constant,
            1 => OpCode::ConstantLong,
            2 => OpCode::Add,
            3 => OpCode::Subtract,
            4 => OpCode::Multiply,
            5 => OpCode::Divide,
            6 => OpCode::Negate,
            7 => OpCode::Return,
            _ => panic!("Unknown opcode {}", value),
        }
    }
}

pub trait AsOpCode {
    fn as_opcode(&self) -> OpCode;
}

impl AsOpCode for u8 {
    fn as_opcode(&self) -> OpCode {
        OpCode::from(*self)
    }
}
