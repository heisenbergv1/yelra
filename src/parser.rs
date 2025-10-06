use crate::ast::Expr;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        // Parse a primary expression first
        let first = match self.next() {
            Some(Token::Number(s)) => {
                let n: f64 = s.parse().map_err(|e| format!("Invalid number '{}': {}", s, e))?;
                Expr::Number(n)
            }
            Some(Token::Symbol(s)) => {
                // A bare symbol (not inside parentheses) — return as symbol
                Expr::Symbol(s)
            }
            Some(Token::LParen) => {
                // parse list until matching RParen
                let mut exprs = Vec::new();
                while let Some(tok) = self.peek() {
                    if matches!(tok, Token::RParen) {
                        // consume RParen
                        self.next();
                        return Ok(Expr::List(exprs));
                    } else {
                        let e = self.parse_expr()?;
                        exprs.push(e);
                    }
                }
                return Err("Unclosed '(' — reached end of input".to_string());
            }
            Some(Token::RParen) => return Err("Unexpected ')'".to_string()),
            None => return Err("Unexpected end of input".to_string()),
        };

        // If the primary expression is a Number or a List, try to parse infix continuation:
        // pattern: first (Symbol op, Expr rhs)+
        match &first {
            Expr::Number(_) | Expr::List(_) => {
                let mut operands: Vec<Expr> = vec![first.clone()];
                let mut ops: Vec<String> = Vec::new();

                // try to collect (op, rhs) pairs
                loop {
                    // if next token is a Symbol, consume it and parse rhs
                    if let Some(Token::Symbol(op)) = self.peek().cloned() {
                        // consume operator
                        self.next();
                        // parse rhs expression
                        let rhs = self.parse_expr()?;
                        ops.push(op);
                        operands.push(rhs);
                    } else {
                        break;
                    }
                }

                if ops.is_empty() {
                    // no infix continuation; just return the primary expr
                    Ok(operands.into_iter().next().unwrap())
                } else if ops.len() == 1 {
                    // single operator: (op first rhs)
                    let op = ops[0].clone();
                    let mut list: Vec<Expr> = Vec::new();
                    list.push(Expr::Symbol(op));
                    list.extend(operands.into_iter());
                    Ok(Expr::List(list))
                } else {
                    // multiple operators: ensure they are all the same
                    let all_same = ops.iter().all(|o| o == &ops[0]);
                    if all_same {
                        let op0 = ops[0].clone();
                        let mut list: Vec<Expr> = Vec::new();
                        list.push(Expr::Symbol(op0));
                        list.extend(operands.into_iter());
                        Ok(Expr::List(list))
                    } else {
                        Err("Mixed operators without parentheses are not supported — use parentheses to disambiguate.".to_string())
                    }
                }
            }
            // If the first expression is a Symbol (e.g. user typed "+ 1 2" without parentheses),
            // don't attempt to treat following tokens as infix; just return the symbol expression.
            Expr::Symbol(_) => Ok(first),
        }
    }
}

// top-level parse entry
pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut p = Parser::new(tokens);
    let expr = p.parse_expr()?;
    if p.peek().is_some() {
        return Err("Extra tokens after first expression".to_string());
    }
    Ok(expr)
}
