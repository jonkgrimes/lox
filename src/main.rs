use std::env;

use lox::{run_file, run_prompt};

fn main() {
    let args: Vec<String>  = env::args().collect();

    match args.len() {
        0 => {
            run_prompt()
        },
        1 => {
            run_file(&args[0])
        }
        _ => { 
            println!("Usage: lox [script]");
            std::process::exit(64)
        }
    }
}
