use crate::bytecode::{chunk::Chunk, error::Result};

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self { chunk, ip: 0 }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<()> {
        todo!()
    }

    fn run(&self) -> Result<()> {
        todo!()
    }
}
