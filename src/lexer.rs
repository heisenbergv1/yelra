// lexer.rs

use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // Numbers (priority beats Symbol)
    #[regex(r"-?[0-9]+(\.[0-9]+)?", |lex| lex.slice().to_string(), priority = 3)]
    Number(String),

    // Operators and Identifiers (merged into one Symbol variant)
    #[regex(r"[+\-*/=<>!]+|[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string(), priority = 2)]
    Symbol(String),

    // Skip whitespace
    #[regex(r"[ \t\r\n]+", logos::skip)]
    Whitespace,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(res) = lexer.next() {
        match res {
            Ok(Token::Whitespace) => continue,
            Ok(tok) => tokens.push(tok),
            Err(_) => {
                let span = lexer.span();
                return Err(format!("Unexpected token at {}..{}", span.start, span.end));
            }
        }
    }

    Ok(tokens)
}
