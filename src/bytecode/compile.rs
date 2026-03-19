use strum_macros::Display;

use crate::{
    PRINT_CODE,
    bytecode::{
        chunk::Chunk,
        common::OpCode,
        error::{Error, ParserError, Result},
        scanner::Scanner,
        token::{Literal, Token, TokenType},
    },
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

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Precedence {
    pub fn next(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

pub type ParseFn<'a> = fn(&mut Compiler<'a>) -> Result<()>;

pub struct ParseRule<'a> {
    pub prefix: Option<ParseFn<'a>>,
    pub infix: Option<ParseFn<'a>>,
    pub precedence: Precedence,
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
        self.expression()?;
        self.consume(TokenType::EOF, "Expect end of expression")?;
        self.end_compiler();

        Ok(())
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();

        if PRINT_CODE {
            self.current_chunk().disassemble_chunk("code");
        }
    }

    pub fn binary(&mut self) -> Result<()> {
        let op_type = self.parser.previous.token_type;
        let precedence = op_type.precedence();
        self.parse_precedence(precedence.next())?;

        match op_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            _ => {}
        }

        Ok(())
    }

    pub fn grouping(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after expression")
    }

    pub fn number(&mut self) -> Result<()> {
        let literal = self.parser.previous.literal.as_ref();
        if let Some(Literal::Number(value)) = literal {
            self.emit_constant(*value);
        }
        Ok(())
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    pub fn unary(&mut self) -> Result<()> {
        let operator_type = self.parser.previous.token_type;

        self.parse_precedence(Precedence::Unary)?;

        match operator_type {
            TokenType::Minus => {
                self.emit_byte(OpCode::Negate as u8);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn expression(&mut self) -> Result<()> {
        self.parse_precedence(Precedence::Assignment)?;

        Ok(())
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<()> {
        self.advance()?;
        let token = &self.parser.previous;

        let prefix_rule = token
            .token_type
            .prefix_rule()
            .ok_or(Error::Parser(ParserError::new(token, "Expect expression.")))?;

        prefix_rule(self)?;

        let current = self.parser.current.token_type;

        while precedence <= current.precedence() {
            self.advance()?;

            let infix_rule = self.parser.previous.token_type.infix_rule();
            if let Some(rule) = infix_rule {
                rule(self)?;
            }
        }

        Ok(())
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
