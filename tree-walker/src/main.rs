#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]

mod interpreter_error;
mod scanner;

use std::{
    env, fs,
    io::{self, stdin, stdout, Write},
    process::exit,
};

use scanner::Scanner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<String>>();

    match args.len() {
        2.. => {
            println!("Usage: rlox [script]");
            exit(64);
        }
        1 => run_file(&args[0])?,
        0 => run_prompt(),
    }

    Ok(())
}

/// # Errors
/// Will return an error if it's unable to read an input file
pub fn run_file(path: &str) -> io::Result<()> {
    let source = fs::read_to_string(path)?;

    if run(source).is_err() {
        exit(65);
    };

    Ok(())
}

/// # Panics
/// Will panic if stdout fails to flush or if stdin fails to read.
pub fn run_prompt() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        _ = run(line);
    }
}

fn run(source: String) -> Result<(), ()> {
    let scanner = Scanner::new(source);

    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(errors) => {
            for err in errors {
                eprintln!("{err}");
            }
            return Err(());
        }
    };

    println!("{tokens:?}");

    Ok(())
}
