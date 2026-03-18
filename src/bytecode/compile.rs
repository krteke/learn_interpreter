use crate::bytecode::{error::Result, scanner::Scanner, token::TokenType};

pub fn compile(source: &str) -> Result<()> {
    let mut scanner = Scanner::new(source.as_bytes());
    let mut line = -1;
    loop {
        let token = scanner.scan_token()?;

        if token.line != line {
            print!("{:4}", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:2} '{}'", token.token_type, token.lex);

        if let TokenType::EOF = token.token_type {
            break;
        }
    }

    Ok(())
}
