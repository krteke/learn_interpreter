use crate::{
    DEBUG,
    bytecode::{
        chunk::Chunk,
        common::{AsOpCode, OpCode},
        compile::Compiler,
        error::{Error, Result, RuntimeError},
        value::{Obj, StringInterner, Value},
    },
};

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub strings: StringInterner,
    pub stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            strings: StringInterner::new(),
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let mut compiler = Compiler::new(source, &mut self.strings);
        compiler.compile()?;
        self.chunk = compiler.chunk;

        self.run()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
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
                OpCode::Nil => self.push(Value::Nil),
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(Value::Bool(a == b));
                }
                OpCode::Add => {
                    let len = self.stack.len();

                    match (&self.stack[len - 1], &self.stack[len - 2]) {
                        (Value::Number(_), Value::Number(_)) | (Value::Obj(_), Value::Obj(_)) => {}
                        _ => {
                            return Err(Error::Runtime(RuntimeError::new(
                                self.ip,
                                "Operands must be two numbers or two strings.",
                            )));
                        }
                    }

                    self.binary_op(instruction)?;
                }
                OpCode::Subtract
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Greater
                | OpCode::Less => {
                    let len = self.stack.len();

                    match (&self.stack[len - 1], &self.stack[len - 2]) {
                        (Value::Number(_), Value::Number(_)) => {}
                        _ => {
                            return Err(Error::Runtime(RuntimeError::new(
                                self.ip,
                                "Operands must be numbers.",
                            )));
                        }
                    }

                    self.binary_op(instruction)?;
                }
                OpCode::Not => {
                    let value = self.pop();
                    self.push(!value);
                }
                OpCode::Return => {
                    let result = self.pop();
                    println!("{}", result);

                    return Ok(());
                }
                OpCode::Negate => {
                    let value = self.stack.last_mut();
                    if let Some(Value::Number(v)) = value {
                        *v = -*v;
                    } else {
                        return Err(Error::Runtime(RuntimeError::new(
                            self.ip,
                            "Operand must be a number.",
                        )));
                    }
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.chunk.code[self.ip - 1]
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;

        self.chunk.constants.values[index].clone()
    }

    fn binary_op(&mut self, op: OpCode) -> Result<()> {
        let b = self.pop();
        let a = self.pop();

        match op {
            OpCode::Add => match (&a, &b) {
                (Value::Obj(a), Value::Obj(b)) => {
                    let (Obj::String(a), Obj::String(b)) = (a, b);
                    let mut value = Vec::with_capacity(a.len() + b.len());
                    value.extend_from_slice(a.as_bytes());
                    value.extend_from_slice(b.as_bytes());

                    let str = self.strings.intern(&String::from_utf8(value).unwrap());
                    self.push(Value::Obj(Obj::String(str)));
                }
                _ => self.push(a + b),
            },
            OpCode::Subtract => self.push(a - b),
            OpCode::Multiply => self.push(a * b),
            OpCode::Divide => self.push(a / b),
            OpCode::Greater => self.push(Value::Bool(a > b)),
            OpCode::Less => self.push(Value::Bool(a < b)),
            _ => unreachable!(),
        }

        Ok(())
    }

    fn read_constant_long(&mut self) -> Value {
        let byte1 = self.read_byte();
        let byte2 = self.read_byte();
        let byte3 = self.read_byte();

        let constant = u32::from_be_bytes([0, byte1, byte2, byte3]);

        self.chunk.constants.values[constant as usize].clone()
    }
}
