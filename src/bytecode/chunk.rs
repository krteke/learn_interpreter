use crate::bytecode::{common::OpCode, value::ValueArray};

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.values.push(value);
        self.constants.values.len() - 1
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==\n", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        println!("{:04} ", offset);

        let instruction = self.code[offset].into();
        match instruction {
            OpCode::Return => {
                return simple_instruction("Return", offset);
            }
            _ => {}
        }

        todo!()
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}\n", name);
    offset + 1
}
