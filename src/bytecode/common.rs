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
    GetLocal = 8,
    SetLocal = 9,
    GetGlobal = 10,
    DefineGlobal = 11,
    SetGlobal = 12,
    Greater = 13,
    Less = 14,
    Subtract = 15,
    Multiply = 16,
    Divide = 17,
    Not = 18,
    Negate = 19,
    Print = 20,
    Jump = 21,
    JumpIfFalse = 22,
    Loop = 23,
    Return = 24,
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
            8 => OpCode::GetLocal,
            9 => OpCode::SetLocal,
            10 => OpCode::GetGlobal,
            11 => OpCode::DefineGlobal,
            12 => OpCode::SetGlobal,
            13 => OpCode::Greater,
            14 => OpCode::Less,
            15 => OpCode::Subtract,
            16 => OpCode::Multiply,
            17 => OpCode::Divide,
            18 => OpCode::Not,
            19 => OpCode::Negate,
            20 => OpCode::Print,
            21 => OpCode::Jump,
            22 => OpCode::JumpIfFalse,
            23 => OpCode::Loop,
            24 => OpCode::Return,
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
