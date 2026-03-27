use std::io::Write;

use crate::bytecode::{
    error::{Error, Result},
    vm::VM,
};

mod ast;
mod bytecode;

pub const DEBUG: bool = cfg!(feature = "debug_trace_execution");
pub const PRINT_CODE: bool = cfg!(feature = "debug_print_code");

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() == 1 {
        repl()?;
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        println!("Usage: rlox [path]");
        std::process::exit(64);
    }

    Ok(())
}

fn repl() -> Result<()> {
    let mut input = String::new();
    let mut vm = VM::new();

    loop {
        print!("> ");
        std::io::stdout().flush().map_err(Error::Io)?;
        std::io::stdin().read_line(&mut input).map_err(Error::Io)?;

        if let Err(e) = vm.interpret(&input) {
            eprintln!("error: {}", e);
        }
        input.clear();
    }
}

fn run_file(path: &str) -> Result<()> {
    let source = std::fs::read_to_string(path).map_err(Error::Io)?;
    let mut vm = VM::new();
    vm.interpret(&source)?;

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
