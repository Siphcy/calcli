mod sql_init;

use meval::{eval_str, Expr, Context};
use std::io::{self, Write};

fn main() {
    println!("Calculator CLI - Enter expressions to evaluate (type 'quit' to exit)");
    let mut parse_result: Vec<f64> = Vec::new();
    let mut counter = 1;
        let mut ctx = Context::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }


        if input.is_empty() {
            continue;
        }

        if input.starts_with("let ") {
            match parse_let_statement(input) {
                Some((name, value)) => {
                    ctx.var(name, value);
                    println!("{} = {}", name, value);
                }
                None => eprintln!("Invalid let syntax. Use: x = #"),
            }
            continue;
        }

        for n in 1..counter {
            let pattern = format!("ln{}", n);
            if input.contains(&pattern) {
                if parse_result.get(n-1) != None {
                    ctx.var(pattern, parse_result[n-1]);
                }
            }


        }
         match input.parse::<Expr>().and_then(|e| e.eval_with_context(&ctx)) {
              Ok(result) => {
                parse_result.push(result);
                println!("{}) = {}", counter, result);
                counter += 1;
            }
              Err(e) => eprintln!("Error: {}", e),
          }
    }
}

fn parse_let_statement(input: &str) -> Option<(&str, f64)> {

    let rest = input.strip_prefix("let ")?;

    let parts: Vec<&str> = rest.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None
    }
    let var_name = parts[0].trim();
    let value_str = parts[1].trim();

    let value = eval_str(value_str).ok()?;

    Some((var_name, value))
}

fn evaluate_expression(expr: &str) -> Result<f64, meval::Error> {
    eval_str(expr)
}

fn evaluate_with_variables(expr: &str) -> Result<f64, meval::Error> {
    let parsed: Expr = expr.parse()?;

    let mut ctx = Context::new();
    ctx.var("x", 2.0);
    ctx.var("y", 3.0);

    parsed.eval_with_context(&ctx)
}
