#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::Write;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

mod scanner;
mod token;
mod expr;
mod parser;

use scanner::{Scanner};
use parser::{Parser};

pub fn run_prompt() -> io::Result<()> {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        run(&input)?
    }
}

pub fn run_file(path: &String) -> io::Result<()> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    run(&contents)
}

fn run(source: &String) -> io::Result<()> {
    let mut scanner: Scanner = Scanner::new(source.clone());
    match scanner.scan() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens.to_vec());
            let expr = parser.parse();
            println!("{}", expr);
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