#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    ConstantLong = 1,
    Nil = 2,
    True = 3,
    False = 4,
    Add = 5,
    Equal = 6,
    Greater = 7,
    Less = 8,
    Subtract = 9,
    Multiply = 10,
    Divide = 11,
    Not = 12,
    Negate = 13,
    Return = 14,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Constant,
            1 => OpCode::ConstantLong,
            2 => OpCode::Nil,
            3 => OpCode::True,
            4 => OpCode::False,
            5 => OpCode::Add,
            6 => OpCode::Equal,
            7 => OpCode::Greater,
            8 => OpCode::Less,
            9 => OpCode::Subtract,
            10 => OpCode::Multiply,
            11 => OpCode::Divide,
            12 => OpCode::Not,
            13 => OpCode::Negate,
            14 => OpCode::Return,
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
