#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::Write;

mod scanner;
mod token;
mod expr;

use scanner::{Scanner};
use token::Token;
use expr::{Expr, Binary, Unary, Literal, Grouping};

pub fn run_prompt() -> io::Result<()> {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        run(&input)
    }
}

pub fn run_file(path: &String) {
    println!("{}", path);
}

fn run(source: &String) {
    let mut scanner: Scanner = Scanner::new(source.clone());
    scanner.scan();
    let tokens: Vec<Token> = scanner.tokens;

    for token in tokens {
        println!("=> {}", token);
    }
}

fn error(line: u32, message: &str) {
    report(line, "", message)
}

fn report(line: u32, location: &str, message: &str) {
    eprintln!("[line {line}] Error{location}: {message}", line=line, location=location, message=message)
}