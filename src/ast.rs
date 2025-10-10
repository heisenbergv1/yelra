#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Symbol(String),
    List(Vec<Expr>),
}

pub fn eval(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::List(items) => {
            if items.is_empty() {
                return Err("Empty list".to_string());
            }
            match &items[0] {
                Expr::Symbol(op) => {
                    let args: Result<Vec<f64>, String> = items[1..].iter().map(eval).collect();
                    let args = args?;
                    match op.as_str() {
                        "+" => Ok(args.iter().sum()),
                        "-" => {
                            if args.len() == 1 {
                                Ok(-args[0])
                            } else {
                                Ok(args[0] - args[1..].iter().sum::<f64>())
                            }
                        }
                        "*" => Ok(args.iter().product()),
                        "/" => {
                            if args.len() == 1 {
                                Ok(1.0 / args[0])
                            } else {
                                args[1..].iter().try_fold(args[0], |acc, x| {
                                    if *x == 0.0 {
                                        Err("Division by zero".to_string())
                                    } else {
                                        Ok(acc / x)
                                    }
                                })
                            }
                        }
                        _ => Err(format!("Unknown operator: {}", op)),
                    }
                }
                _ => Err("List must start with a symbol".to_string()),
            }
        }
        Expr::Symbol(s) => Err(format!("Cannot evaluate bare symbol: {}", s)),
    }
}
