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
    println!("Run prompt");
    let expr = Binary::new(
        Box::new(Unary::new(Token::minus(), Box::new(Literal::new(123)))),
        Token::star(),
        Box::new(Grouping::new(Box::new(Literal::new(45.67))))
    );
    expr.print();
    Ok(())
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