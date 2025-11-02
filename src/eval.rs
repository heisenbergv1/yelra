use crate::ast::Expr;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

pub fn eval(expr: &Expr) -> Result<Value, String> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Symbol(s) => Err(format!("Unbound symbol '{}'", s)),
        Expr::List(list) => {
            if list.is_empty() {
                return Err("Cannot evaluate empty list".to_string());
            }
            match &list[0] {
                Expr::Symbol(op) => {
                    // evaluate arguments (we only support numeric values for now)
                    let mut args = Vec::new();
                    for a in &list[1..] {
                        match eval(a)? {
                            Value::Number(n) => args.push(n),
                        }
                    }

                    match op.as_str() {
                        "+" => Ok(Value::Number(args.iter().sum())),
                        "-" => {
                            match args.len() {
                                0 => Err("'-' needs at least one argument".to_string()),
                                1 => Ok(Value::Number(-args[0])),
                                _ => {
                                    let mut res = args[0];
                                    for v in &args[1..] { res -= v; }
                                    Ok(Value::Number(res))
                                }
                            }
                        }
                        "*" => {
                            let mut res = 1.0;
                            for v in &args { res *= v; }
                            Ok(Value::Number(res))
                        }
                        "/" => {
                            match args.len() {
                                0 => Err("'/' needs at least one argument".to_string()),
                                1 => Ok(Value::Number(1.0 / args[0])),
                                _ => {
                                    let mut res = args[0];
                                    for v in &args[1..] {
                                        if *v == 0.0 {
                                            return Err("division by zero".to_string());
                                        }
                                        res /= v;
                                    }
                                    Ok(Value::Number(res))
                                }
                            }
                        }
                        other => Err(format!("Unknown operator '{}'", other))
                    }
                }
                _ => Err("First element of list must be a symbol (operator)".to_string()),
            }
        }
    }
}
