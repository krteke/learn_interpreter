use crate::bytecode::{common::OpCode, value::ValueArray};

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        if self.lines.get(line - 1).is_some() {
            self.lines[line - 1] += 1;
        } else {
            let extend_vec = vec![0; line - self.lines.len() - 1];
            self.lines.extend_from_slice(&extend_vec);
            self.lines.push(1);
        }

        self.code.push(byte);
    }

    pub fn get_line(&self, offset: usize) -> usize {
        let mut o = 0;

        for (i, n) in self.lines.iter().enumerate() {
            o += n;
            if offset < o {
                return i + 1;
            }
        }

        panic!("offset out of bounds: {}", offset);
    }

    pub fn write_constant(&mut self, value: f64, line: usize) {
        let index = self.add_constant(value);

        if index < 256 {
            self.write_chunk(OpCode::Constant as u8, line);
            self.write_chunk(index as u8, line);
        } else {
            self.write_chunk(OpCode::ConstantLong as u8, line);
            let index = (index as u32).to_be_bytes();
            self.write_chunk(index[1], line);
            self.write_chunk(index[2], line);
            self.write_chunk(index[3], line);
        }
    }

    fn add_constant(&mut self, value: f64) -> usize {
        self.constants.values.push(value);
        self.constants.values.len() - 1
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.get_line(offset));
        }

        let instruction = self.code[offset].into();
        match instruction {
            OpCode::Constant => self.constant_instruction("Constant", offset),
            OpCode::ConstantLong => self.constant_long_instruction("ConstantLong", offset),
            OpCode::Return => self.simple_instruction("Return", offset),
            OpCode::Negate => self.simple_instruction("Negate", offset),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);

        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];

        println!(
            "{} {:4} '{}'",
            name, constant, self.constants.values[constant as usize]
        );

        offset + 2
    }

    fn constant_long_instruction(&self, name: &str, offset: usize) -> usize {
        let byte1 = self.code[offset + 1];
        let byte2 = self.code[offset + 2];
        let byte3 = self.code[offset + 3];

        let constant = u32::from_be_bytes([0, byte1, byte2, byte3]);

        println!(
            "{} {:4} '{}'",
            name, constant, self.constants.values[constant as usize]
        );

        offset + 4
    }
}
