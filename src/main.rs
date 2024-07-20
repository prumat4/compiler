mod lexer;
mod parser;

use lexer::lexer::Lexer;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments");
    }

    let content = fs::read_to_string(args[1].clone()).expect("No such file");
    dbg!(&content);

    // let bytes = content.as_bytes();

    let lexer = Lexer::new(content.into());
    dbg!(&lexer);
}
