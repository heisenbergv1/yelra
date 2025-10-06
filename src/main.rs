mod lexer;
mod parser;
mod ast;

use std::io::{self, Write};
use lexer::tokenize;
use parser::parse;

fn main() {
    println!("Lispy v0.1 — type 'exit' or Ctrl+D to quit");

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
                Ok(expr) => println!("AST: {:?}", expr),
                Err(e) => println!("Parse error: {}", e),
            },
            Err(e) => println!("Lex error: {}", e),
        }
    }
}
