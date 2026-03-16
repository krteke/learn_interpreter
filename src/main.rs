// use std::{env, io::Write, process};

// use crate::ast::{
//     error::{Error, Result},
//     interpreter::Interpreter,
//     parser::Parser,
//     scanner::Scanner,
// };

use std::error::Error;

mod ast;
mod bytecode;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// fn main() -> Result<()> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() > 2 {
//         println!("Usage: rlox [script]");
//         process::exit(64);
//     } else if args.len() == 2 {
//         run_file(&args[1])?;
//     } else {
//         run_prompt()?;
//     }

//     Ok(())
// }

// fn run_file(path: &str) -> Result<()> {
//     let source = std::fs::read_to_string(path).map_err(Error::Io)?;
//     let mut interpreter = Interpreter::new();
//     run(&source, &mut interpreter)?;

//     Ok(())
// }

// fn run_prompt() -> Result<()> {
//     let mut input = String::new();
//     let mut interpreter = Interpreter::new();

//     loop {
//         print!("> ");
//         std::io::stdout().flush().map_err(Error::Io)?;

//         let line = std::io::stdin().read_line(&mut input).map_err(Error::Io)?;
//         if line == 0 {
//             break Ok(());
//         }
//         if let Err(e) = run(&input, &mut interpreter) {
//             eprintln!("error: {}", e);
//         };
//         input.clear();
//     }
// }

// fn run(source: &str, interpreter: &mut Interpreter) -> Result<()> {
//     let mut scanner = Scanner::new(source);
//     scanner.scan_tokens()?;

//     let mut parser = Parser::new(scanner.tokens);
//     let expr = parser.parse();

//     if let Ok(expr) = expr {
//         interpreter.interpret(&expr)?;
//     }

//     Ok(())
// }
