use std::borrow::Cow;

use strum_macros::Display;

use crate::bytecode::compile::{Compiler, ParseFn, ParseRule, Precedence};

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lex: Cow<'a, str>,
    pub line: i32,
    pub literal: Option<Literal>,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lex: &'a str, line: i32, literal: Option<Literal>) -> Self {
        Self {
            token_type,
            lex: Cow::Borrowed(lex),
            line,
            literal,
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Display, Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl<'a> TokenType {
    pub fn prefix_rule(&self) -> Option<ParseFn<'a>> {
        match self {
            Self::LeftParen => Some(Compiler::grouping),
            Self::Minus | Self::Bang => Some(Compiler::unary),
            Self::Number => Some(Compiler::number),
            Self::False | Self::True | Self::Nil => Some(Compiler::literal),
            _ => None,
        }
    }

    pub fn infix_rule(&self) -> Option<ParseFn<'a>> {
        match self {
            Self::Minus
            | Self::Plus
            | Self::Star
            | Self::Slash
            | Self::BangEqual
            | Self::EqualEqual
            | Self::Greater
            | Self::GreaterEqual
            | Self::Less
            | Self::LessEqual => Some(Compiler::binary),
            _ => None,
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Self::Minus | Self::Plus => Precedence::Term,
            Self::Star | Self::Slash => Precedence::Factor,
            Self::BangEqual | Self::EqualEqual => Precedence::Equality,
            Self::Greater | Self::GreaterEqual | Self::Less | Self::LessEqual => {
                Precedence::Comparison
            }
            _ => Precedence::None,
        }
    }

    pub fn rule(&self) -> ParseRule<'a> {
        ParseRule {
            prefix: self.prefix_rule(),
            infix: self.infix_rule(),
            precedence: self.precedence(),
        }
    }
}
