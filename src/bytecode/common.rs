#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    ConstantLong = 1,
    Nil = 2,
    True = 3,
    False = 4,
    Add = 5,
    Equal = 6,
    Pop = 7,
    GetGlobal = 8,
    DefineGlobal = 9,
    SetGlobal = 10,
    Greater = 11,
    Less = 12,
    Subtract = 13,
    Multiply = 14,
    Divide = 15,
    Not = 16,
    Negate = 17,
    Print = 18,
    Return = 19,
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
            7 => OpCode::Pop,
            8 => OpCode::GetGlobal,
            9 => OpCode::DefineGlobal,
            10 => OpCode::SetGlobal,
            11 => OpCode::Greater,
            12 => OpCode::Less,
            13 => OpCode::Subtract,
            14 => OpCode::Multiply,
            15 => OpCode::Divide,
            16 => OpCode::Not,
            17 => OpCode::Negate,
            18 => OpCode::Print,
            19 => OpCode::Return,
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
