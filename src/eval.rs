use meval::{eval_str, Expr, Context};

pub fn evaluate_input(
    counter: usize,
    input: &str,
    parse_result: &mut Vec<f64>,
    ctx: &mut Context,
) -> Option<f64> {
    // Empty input - nothing to evaluate
    if input.is_empty() {
        return None;
    }

    // Handle "let x = 5" statements
    if input.starts_with("let ") {
        match parse_let_statement(input) {
            Some((name, value)) => {
                ctx.var(name, value);
                return Some(value);
            }
            None => eprintln!("Invalid let syntax. Use: let x = 5"),
        }
        return None;
    }

    // Register previous results as ln1, ln2, etc.
    for n in 1..counter {
        let pattern = format!("ln{}", n);
        if input.contains(&pattern) {
            if let Some(value) = parse_result.get(n - 1) {
                ctx.var(&pattern, *value);
            }
        }
    }

    // Evaluate the expression
    match input.parse::<Expr>().and_then(|e| e.eval_with_context(ctx)) {
        Ok(result) => {
            parse_result.push(result);
            Some(result)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

pub fn parse_let_statement(input: &str) -> Option<(&str, f64)> {
    let rest = input.strip_prefix("let ")?;

    let parts: Vec<&str> = rest.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }
    let var_name = parts[0].trim();
    let value_str = parts[1].trim();

    let value = eval_str(value_str).ok()?;

    Some((var_name, value))
}
