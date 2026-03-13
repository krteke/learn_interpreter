use std::{collections::HashMap, sync::LazyLock};

use crate::{
    error::{Error, Result, ScanError},
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Scanner {
    pub source: Vec<char>,
    state: ScannerState,
    pub tokens: Vec<Token>,
}

struct ScannerState {
    start: usize,
    current: usize,
    line: usize,
}

static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ])
});

impl Default for ScannerState {
    fn default() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            state: ScannerState::default(),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<()> {
        while !self.at_end() {
            self.state.start = self.state.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.state.line,
            None,
        ));

        Ok(())
    }

    fn at_end(&self) -> bool {
        self.state.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.source[self.state.current];
        self.state.current += 1;

        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => {
                if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek().is_some_and(|c| c != '\n') {
                        self.state.current += 1;
                    }
                    return Ok(());
                } else {
                    TokenType::Slash
                }
            }
            ' ' | '\r' | '\t' => return Ok(()),
            '\n' => {
                self.state.line += 1;
                return Ok(());
            }
            '"' => {
                while self.peek().is_some_and(|c| c != '"') {
                    if self.source[self.state.current] == '\n' {
                        self.state.line += 1;
                    }
                    self.state.current += 1;
                }

                if self.at_end() {
                    return Err(Error::Scan(ScanError::new(
                        self.state.line,
                        "Unterminated string".to_string(),
                    )));
                }

                let literal: String = self.source[self.state.start + 1..self.state.current]
                    .iter()
                    .collect();
                self.state.current += 1;
                self.add_token(TokenType::String, Some(Literal::String(literal)));
                return Ok(());
            }
            _ => {
                if c.is_ascii_digit() {
                    return self.process_number();
                } else if c.is_ascii_alphabetic() {
                    return self.process_identifier();
                } else {
                    return Err(Error::Scan(ScanError::new(
                        self.state.line,
                        format!("Unexpected character: {}", c),
                    )));
                }
            }
        };

        self.add_token(token_type, None);

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let state = &self.state;

        let start = state.start;
        let current = state.current;
        let line = state.line;

        let text = self.source[start..current].iter().collect();
        self.tokens
            .push(Token::new(token_type, text, line, literal));
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.peek().is_some_and(|char| char != c) {
            return false;
        }

        self.state.current += 1;
        true
    }

    fn process_number(&mut self) -> Result<()> {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.state.current += 1;
        }

        if self.peek().is_some_and(|c| c == '.')
            && self.peek_next().is_some_and(|c| c.is_ascii_digit())
        {
            self.state.current += 1;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.state.current += 1;
            }
        }

        let num: String = self.source[self.state.start..self.state.current]
            .iter()
            .collect();
        if let Ok(num) = num.parse::<f64>() {
            self.add_token(TokenType::Number, Some(Literal::Number(num)));

            Ok(())
        } else {
            Err(Error::Scan(ScanError::new(
                self.state.line,
                format!("Unexpected string: {}", num),
            )))
        }
    }

    fn process_identifier(&mut self) -> Result<()> {
        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric()) {
            self.state.current += 1;
        }

        let text: String = self.source[self.state.start..self.state.current]
            .iter()
            .collect();

        let token_type = *KEYWORDS
            .get(text.as_str())
            .unwrap_or(&TokenType::Identifier);

        self.add_token(token_type, None);

        Ok(())
    }

    fn peek(&self) -> Option<char> {
        (!self.at_end()).then(|| self.source[self.state.current])
    }

    fn peek_next(&self) -> Option<char> {
        (self.state.current + 1 < self.source.len()).then(|| self.source[self.state.current + 1])
    }
}
