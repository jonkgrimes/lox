use std::env;
use std::io;

use lox::{run_file, run_prompt};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: lox [script]");
            std::process::exit(64)
        }
    }
}
