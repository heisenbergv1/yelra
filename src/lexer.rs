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

    // Symbols (operators and identifiers)
    #[regex(r"[A-Za-z_+\-*/=<>!][A-Za-z0-9_+\-*/=<>!]*", |lex| lex.slice().to_string())]
    Symbol(String),

    // A variant matched by logos; we'll drop instances of it inside tokenize()
    #[regex(r"[ \t\r\n]+", logos::skip)]
    Whitespace,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(res) = lexer.next() {
        match res {
            // logos::skip should skip, but some logos versions may still yield the variant.
            // defensively ignore it here.
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
