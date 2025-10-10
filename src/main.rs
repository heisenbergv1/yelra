// src/main.rs

mod ast;
mod lexer;
mod parser;

use lexer::tokenize;
use parser::parse;
use std::io::{self, Write};

fn main() {
    println!("yelra v0.1 — type 'exit' or Ctrl+D to quit");

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break; // EOF
        }
        let input = input.trim();
        if input == "exit" {
            break;
        }

        match tokenize(input) {
            Ok(tokens) => match parse(tokens) {
                Ok(expr) => match ast::eval(&expr) {
                    Ok(val) => println!("{}", val),
                    Err(e) => println!("Eval error: {}", e),
                },
                Err(e) => println!("Parse error: {}", e),
            },
            Err(e) => println!("Lex error: {}", e),
        }
    }
}
