use std::{collections::HashMap, sync::LazyLock};

use crate::bytecode::{
    error::{Result, ScanError, VMError},
    token::{Literal, Token, TokenType},
};

pub struct Scanner<'a> {
    pub source: &'a [u8],
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| HashMap::from([]));

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Result<Token<'a>> {
        self.start = self.current;
        self.skip_whitespace();

        if self.at_end() {
            return Ok(Token::new(TokenType::EOF, "", self.line, None));
        }

        let c = self.advance();

        let token_type = match c {
            b'(' => TokenType::LeftParen,
            b')' => TokenType::RightParen,
            b'{' => TokenType::LeftBrace,
            b'}' => TokenType::RightBrace,
            b',' => TokenType::Comma,
            b'.' => TokenType::Dot,
            b'-' => TokenType::Minus,
            b'+' => TokenType::Plus,
            b';' => TokenType::Semicolon,
            b'*' => TokenType::Star,
            b'/' => TokenType::Slash,
            b'!' => {
                if self.match_byte(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            b'=' => {
                if self.match_byte(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            b'<' => {
                if self.match_byte(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            b'>' => {
                if self.match_byte(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            b'"' => {
                return self.string();
            }
            _ => {
                if c.is_ascii_digit() {
                    return self.number();
                } else if is_alpha(c) {
                    return self.identifier();
                } else {
                    return Err(VMError::Scan(ScanError::new(
                        "Unexpected character".to_string(),
                        self.line as usize,
                    )));
                }
            }
        };

        let lex = str::from_utf8(&self.source[self.start..self.current]).unwrap();

        Ok(Token::new(token_type, lex, self.line, None))
    }

    fn identifier(&mut self) -> Result<Token<'a>> {
        while is_alpha(self.peek()) {
            self.advance();
        }

        let lex = str::from_utf8(&self.source[self.start..self.current]).unwrap();
        let token_type = identifier_type(lex);

        Ok(Token::new(token_type, lex, self.line, None))
    }

    fn string(&mut self) -> Result<Token<'a>> {
        while self.peek() != b'"' {
            if self.source[self.current] == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            return Err(VMError::Scan(ScanError::new(
                "Unterminated string".to_string(),
                self.line as usize,
            )));
        }

        self.advance();
        let lex = str::from_utf8(&self.source[self.start..self.current]).unwrap();
        let literal = Literal::String(lex[1..lex.len() - 1].to_string());

        Ok(Token::new(TokenType::String, lex, self.line, Some(literal)))
    }

    fn number(&mut self) -> Result<Token<'a>> {
        self.consume_number();

        if self.peek() == b'.' && self.peek_next().is_some_and(|n| n.is_ascii_digit()) {
            self.advance();
            self.consume_number();
        }

        let lex = str::from_utf8(&self.source[self.start..self.current]).unwrap();
        let literal = Literal::Number(lex.parse::<f64>().unwrap());

        Ok(Token::new(TokenType::Number, lex, self.line, Some(literal)))
    }

    fn consume_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
    }

    fn peek_next(&self) -> Option<u8> {
        (self.current + 1 < self.source.len()).then(|| self.source[self.current + 1])
    }

    fn match_byte(&mut self, c: u8) -> bool {
        if self.at_end() {
            return false;
        }

        if self.peek() != c {
            return false;
        }

        self.advance();
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' => {
                    if self.peek_next().is_some_and(|n| n == b'/') {
                        while self.peek() != b'\n' && !self.at_end() {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> u8 {
        self.source[self.current]
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }
}

fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_' || c >= 128
}

fn identifier_type(s: &str) -> TokenType {
    match s {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}
