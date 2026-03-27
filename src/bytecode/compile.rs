use std::{borrow::Cow, rc::Rc};

use strum_macros::Display;

use crate::{
    PRINT_CODE,
    bytecode::{
        chunk::Chunk,
        common::OpCode,
        error::{Error, ParserError, Result},
        local::Local,
        scanner::Scanner,
        token::{Literal, Token, TokenType},
        value::{Obj, StringInterner, Value},
    },
};

const MAX_LOCALS: usize = u8::MAX as usize + 1;

pub struct Compiler<'a, 'i> {
    pub parser: Parser<'a>,
    pub scanner: Scanner<'a>,
    pub chunk: Chunk,
    pub strings: &'i mut StringInterner,
    pub locals: Vec<Local>,
    pub scope_depth: i32,
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

pub type ParseFn<'a, 'i> = fn(&mut Compiler<'a, 'i>, bool) -> Result<()>;

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let current = Token::new(TokenType::EOF, "", 0, None);
        let previous = Token::new(TokenType::EOF, "", 0, None);

        Self { current, previous }
    }
}

impl<'a, 'i> Compiler<'a, 'i> {
    pub fn new(source: &'a str, strings: &'i mut StringInterner) -> Self {
        let parser = Parser::new();
        let scanner = Scanner::new(source.as_bytes());
        let chunk = Chunk::new();
        let locals = Vec::new();
        let scope_depth = 0;

        Self {
            parser,
            scanner,
            chunk,
            strings,
            locals,
            scope_depth,
        }
    }

    pub fn compile(&mut self) -> Result<()> {
        self.advance()?;

        while !self.match_type(TokenType::EOF)? {
            self.declaration()?;
        }
        self.end_compiler();

        Ok(())
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();

        if PRINT_CODE {
            self.current_chunk().disassemble_chunk("code");
        }
    }

    fn begin_scpoe(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
    }

    fn synchronize(&mut self) {
        use TokenType::*;

        while self.parser.current.token_type == TokenType::EOF {
            if self.parser.previous.token_type == TokenType::Semicolon {
                return;
            }
            match self.parser.current.token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => {}
            }
            self.advance().ok();
        }
    }

    fn declaration(&mut self) -> Result<()> {
        let result = || -> Result<()> {
            if self.match_type(TokenType::Var)? {
                self.var_declaration()
            } else {
                self.statement()
            }
        }();

        if result.is_err() {
            self.synchronize();
        }

        Ok(())
    }

    fn var_declaration(&mut self) -> Result<()> {
        let global = self.parse_variable("Expect variable name.")?;

        if self.match_type(TokenType::Equal)? {
            self.expression()?;
        } else {
            self.emit_byte(OpCode::Nil as u8);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        self.define_variable(global);

        Ok(())
    }

    fn statement(&mut self) -> Result<()> {
        match self.parser.current.token_type {
            TokenType::Print => {
                self.advance()?;
                self.print_statement()
            }
            TokenType::LeftBrace => {
                self.advance()?;
                self.block()?;
                self.end_scope();

                Ok(())
            }
            _ => self.expression_statement(),
        }
    }

    fn block(&mut self) -> Result<()> {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::EOF) {
            self.declaration()?;
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")
    }

    fn expression_statement(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        self.emit_byte(OpCode::Pop as u8);

        Ok(())
    }

    fn print_statement(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        self.emit_byte(OpCode::Print as u8);

        Ok(())
    }

    fn match_type(&mut self, t: TokenType) -> Result<bool> {
        if !self.check(t) {
            return Ok(false);
        }
        self.advance()?;

        Ok(true)
    }

    fn check(&self, t: TokenType) -> bool {
        self.parser.current.token_type == t
    }

    fn named_variable(&mut self, can_assign: bool) -> Result<()> {
        let mut arg = self.resolve_local(&self.parser.previous)?;

        let (get_op, set_op) = if arg != -1 {
            (OpCode::GetLocal, OpCode::SetLocal)
        } else {
            arg = self.identifier_constant()? as i32;
            (OpCode::GetGlobal, OpCode::SetGlobal)
        };

        if self.match_type(TokenType::Equal)? && can_assign {
            self.expression()?;
            self.emit_bytes(set_op as u8, arg as u8);
        } else {
            self.emit_bytes(get_op as u8, arg as u8);
        }

        Ok(())
    }

    fn resolve_local(&self, token: &Token) -> Result<i32> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == token.lex {
                if local.depth == -1 {
                    return Err(Error::Parser(ParserError::new(
                        token,
                        "Can't read local variable in its own initializer.",
                    )));
                }
                return Ok(i as i32);
            }
        }

        Ok(-1)
    }

    pub fn variable(&mut self, can_assign: bool) -> Result<()> {
        self.named_variable(can_assign)
    }

    pub fn binary(&mut self, can_assign: bool) -> Result<()> {
        let op_type = self.parser.previous.token_type;
        let precedence = op_type.precedence();
        self.parse_precedence(precedence.next())?;

        match op_type {
            TokenType::BangEqual => self.emit_bytes(OpCode::Equal as u8, OpCode::Not as u8),
            TokenType::EqualEqual => self.emit_byte(OpCode::Equal as u8),
            TokenType::Greater => self.emit_byte(OpCode::Greater as u8),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::Less as u8, OpCode::Not as u8),
            TokenType::Less => self.emit_byte(OpCode::Less as u8),
            TokenType::LessEqual => self.emit_bytes(OpCode::Greater as u8, OpCode::Not as u8),
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            _ => {}
        }

        Ok(())
    }

    pub fn grouping(&mut self, can_assign: bool) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after expression")
    }

    pub fn number(&mut self, can_assign: bool) -> Result<()> {
        let literal = self.parser.previous.literal.as_ref();
        if let Some(value) = literal {
            self.emit_constant(value.into());
        }
        Ok(())
    }

    pub fn string(&mut self, can_assign: bool) -> Result<()> {
        if let Some(Literal::String(s)) = &self.parser.previous.literal {
            let interned = self.strings.intern(s);
            self.emit_constant(Value::Obj(Obj::String(interned)));
            Ok(())
        } else {
            Err(Error::Parser(ParserError::new(
                &self.parser.previous,
                "Expect string literal",
            )))
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    pub fn unary(&mut self, can_assign: bool) -> Result<()> {
        let operator_type = self.parser.previous.token_type;

        self.parse_precedence(Precedence::Unary)?;

        match operator_type {
            TokenType::Minus => {
                self.emit_byte(OpCode::Negate as u8);
                Ok(())
            }
            TokenType::Bang => {
                self.emit_byte(OpCode::Not as u8);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn literal(&mut self, can_assign: bool) -> Result<()> {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::False as u8),
            TokenType::Nil => self.emit_byte(OpCode::Nil as u8),
            TokenType::True => self.emit_byte(OpCode::True as u8),
            _ => unreachable!(),
        }

        Ok(())
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

        let can_assign = precedence <= Precedence::Assignment;
        prefix_rule(self, can_assign)?;

        while precedence <= self.parser.current.token_type.precedence() {
            self.advance()?;

            let infix_rule = self.parser.previous.token_type.infix_rule();
            if let Some(rule) = infix_rule {
                rule(self, can_assign)?;
            }

            if can_assign && self.match_type(TokenType::Equal)? {
                return Err(Error::Parser(ParserError::new(
                    &self.parser.current,
                    "Invalid assignment target.",
                )));
            }
        }

        Ok(())
    }

    fn identifier_constant(&mut self) -> Result<u8> {
        let name = self.parser.previous.lex.as_ref();
        let value = Value::Obj(Obj::String(Rc::from(name)));

        let index = u8::try_from(self.chunk.add_constant(value)).expect("Too many constants.");

        Ok(index)
    }

    fn parse_variable(&mut self, err_msg: &str) -> Result<u8> {
        self.consume(TokenType::Identifier, err_msg)?;

        self.declare_variable()?;
        if self.scope_depth > 0 {
            return Ok(0);
        }

        self.identifier_constant()
    }

    fn declare_variable(&mut self) -> Result<()> {
        if self.scope_depth == 0 {
            return Ok(());
        }
        let token = self.parser.previous.clone();
        for local in self.locals.iter().rev() {
            if local.depth != -1 && local.depth < self.scope_depth {
                break;
            }

            if local.name != token.lex {
                return Err(Error::Parser(ParserError::new(
                    &token,
                    "Already a variable with this name in this scope",
                )));
            }
        }

        self.add_local(&token)
    }

    fn add_local(&mut self, token: &Token) -> Result<()> {
        let local = Local::new(token.lex.to_string(), -1);
        if self.locals.len() == MAX_LOCALS {
            return Err(Error::Parser(ParserError::new(token, "Too many locals")));
        }
        self.locals.push(local);

        Ok(())
    }

    fn make_init(&mut self) {
        if let Some(l) = self.locals.last_mut() {
            l.depth = self.scope_depth;
        }
    }

    fn define_variable(&mut self, global: u8) {
        if self.scope_depth > 0 {
            self.make_init();
            return;
        }
        self.emit_bytes(OpCode::DefineGlobal as u8, global);
    }

    fn emit_constant(&mut self, value: Value) {
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
