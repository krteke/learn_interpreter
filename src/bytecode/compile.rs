use strum_macros::Display;

use crate::bytecode::{
    chunk::Chunk,
    common::OpCode,
    error::{Error, ParserError, Result},
    scanner::Scanner,
    token::{Token, TokenType},
};

pub struct Compiler<'a> {
    parser: Parser<'a>,
    scanner: Scanner<'a>,
    chunk: Chunk,
}

pub struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
}

#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let current = Token::new(TokenType::EOF, "", 0, None);
        let previous = Token::new(TokenType::EOF, "", 0, None);

        Self { current, previous }
    }
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        let parser = Parser::new();
        let scanner = Scanner::new(source.as_bytes());
        let chunk = Chunk::new();

        Self {
            parser,
            scanner,
            chunk,
        }
    }

    pub fn compile(&mut self) -> Result<()> {
        self.advance()?;
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression")
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn binary(&mut self) {
        let op_type = self.parser.previous.token_type;

        // self.parse_precedence(precedence);
        match op_type {
            TokenType::Plus => {}
            TokenType::Minus => {}
            _ => {}
        }
        todo!()
    }

    fn grouping(&mut self) -> Result<()> {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression")
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn unary(&mut self) -> Result<()> {
        let operator_type = self.parser.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => {
                self.emit_byte(OpCode::Negate as u8);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        todo!()
    }

    fn emit_constant(&mut self, value: f64) {
        self.chunk
            .write_constant(value, self.parser.previous.line as usize);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk
            .write_chunk(byte, self.parser.previous.line as usize);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn advance(&mut self) -> Result<()> {
        self.parser.previous =
            std::mem::replace(&mut self.parser.current, self.scanner.scan_token()?);

        Ok(())
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<()> {
        if self.parser.current.token_type == token_type {
            self.advance()?;
            return Ok(());
        }

        Err(Error::Parser(ParserError::new(
            &self.parser.current,
            message,
        )))
    }
}
