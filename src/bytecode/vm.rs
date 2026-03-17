use crate::{
    DEBUG,
    bytecode::{chunk::Chunk, common::OpCode, error::Result},
};

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: usize,
    pub stack: Vec<f64>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<()> {
        // self.chunk = chunk;
        // self.ip = 0;

        self.run()
    }

    fn push(&mut self, value: f64) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> f64 {
        self.stack.pop().unwrap()
    }

    fn run(&mut self) -> Result<()> {
        loop {
            if DEBUG {
                print!("        ");
                self.stack.iter().for_each(|s| {
                    print!("[ {} ]", s);
                });
                println!();

                self.chunk.disassemble_instruction(self.ip);
            }

            let instruction = self.read_byte();

            match instruction.into() {
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                OpCode::ConstantLong => {
                    let constant = self.read_constant_long();
                    self.push(constant);
                }
                OpCode::Return => {
                    let result = self.pop();
                    println!("{}", result);

                    return Ok(());
                }
                OpCode::Negate => {
                    let value = self.pop();
                    self.push(-value);
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.chunk.code[self.ip - 1]
    }

    fn read_constant(&mut self) -> f64 {
        self.chunk.constants.values[self.read_byte() as usize]
    }

    fn read_constant_long(&mut self) -> f64 {
        let byte1 = self.read_byte();
        let byte2 = self.read_byte();
        let byte3 = self.read_byte();

        let constant = u32::from_be_bytes([0, byte1, byte2, byte3]);

        self.chunk.constants.values[constant as usize]
    }
}
