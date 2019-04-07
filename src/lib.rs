#[macro_use]
extern crate lazy_static;
extern crate rustyline;

use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod scanner;
mod token;
mod expr;
mod stmt;
mod parser;
mod interpreter;
mod lox_value;
mod lox_error;
mod environment;

use scanner::{Scanner};
use parser::{Parser};
use interpreter::{Interpreter};

pub fn run_prompt() -> io::Result<()> {
    let mut rl = Editor::<()>::new();
    rl.load_history("~/.lox_history").ok();
    loop {
        let readline = rl.readline("lox > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                run(&line)?
            },
            Err(ReadlineError::Interrupted) => {
                println!("Exiting...");
                break
            },
            Err(err) => {
                eprintln!("Unrecoverable error: {:?}", err);
                break
            }
        }
    }

    rl.save_history("~/.lox_history").ok();
    Ok(())
}

pub fn run_file(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    run(&contents)
}

fn run(source: &str) -> io::Result<()> {
    let mut scanner: Scanner = Scanner::new(source.to_string());
    match scanner.scan() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens.to_vec());
            let mut interpreter = Interpreter::new();
            let statements = parser.parse();
            interpreter.interpret(statements)
        },
        Err(e) => error(e.line(), e.description())
    }
    Ok(())
}

fn error(line: u32, message: &str) {
    report(line, "", message)
}

fn report(line: u32, location: &str, message: &str) {
    eprintln!("[line {line}] Error{location}: {message}", line=line, location=location, message=message)
}