use std::borrow::Cow;

use strum_macros::Display;

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
