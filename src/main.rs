mod lexer;
mod parser;

use std::env;
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments");
    }

    let content = fs::read_to_string(args[1].clone()).expect("No such file");
    println!("{}", content);

}
