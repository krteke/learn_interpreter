use crate::{
    DEBUG,
    bytecode::{
        chunk::Chunk,
        common::{AsOpCode, OpCode},
        compile::Compiler,
        error::Result,
        scanner::Scanner,
    },
};

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<f64>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let mut compiler = Compiler::new(source);
        compiler.compile()?;

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

            let instruction = self.read_byte().as_opcode();

            match instruction {
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                OpCode::ConstantLong => {
                    let constant = self.read_constant_long();
                    self.push(constant);
                }
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    self.binary_op(instruction)
                }
                OpCode::Return => {
                    let result = self.pop();
                    println!("{}", result);

                    return Ok(());
                }
                OpCode::Negate => {
                    // let value = self.pop();
                    // self.push(-value);
                    let value = self.stack.last_mut();
                    if let Some(v) = value {
                        *v = -*v;
                    }
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.chunk.code[self.ip - 1]
    }

    fn read_constant(&mut self) -> f64 {
        let index = self.read_byte() as usize;

        self.chunk.constants.values[index]
    }

    fn binary_op(&mut self, op: OpCode) {
        let b = self.pop();
        let a = self.pop();

        match op {
            OpCode::Add => self.push(a + b),
            OpCode::Subtract => self.push(a - b),
            OpCode::Multiply => self.push(a * b),
            OpCode::Divide => self.push(a / b),
            _ => unreachable!(),
        }
    }

    fn read_constant_long(&mut self) -> f64 {
        let byte1 = self.read_byte();
        let byte2 = self.read_byte();
        let byte3 = self.read_byte();

        let constant = u32::from_be_bytes([0, byte1, byte2, byte3]);

        self.chunk.constants.values[constant as usize]
    }
}
