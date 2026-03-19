use crate::unit_conversion::{UNITS_MAGNITUDES, UNITS_KNOWN};
use meval::{eval_str, Expr, Context};

pub fn evaluate_input(
    counter: usize,
    init_input: &str,
    parse_result: &mut Vec<f64>,
    ctx: &mut Context,
) -> Option<f64> {
    // Inits input
    let input: &str = &init_input;

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
            if let Some(&value) = parse_result.get(n - 1) {
                parse_result.push(value);
                ctx.var(&pattern, value);
            }
        }
    }

    //Checks for any unit patterns
    for (m, x) in UNITS_MAGNITUDES.iter() {
        for v in UNITS_KNOWN.iter() {
            let pattern = format!("{}{}", m, v);
            if input.contains(&pattern) {
                ctx.var(&pattern, *x);
            }
        }
    }

    let input: &str = &insert_implicit_multiplication(&init_input);

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


  fn insert_implicit_multiplication(input: &str) -> String {
      let mut result = String::new();
      let mut chars = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .peekable();

      if chars.peek() == Some(&'.'){
        result.push('0');
        }

      while let Some(c) = chars.next() {
          result.push(c);


          if let Some(&next) = chars.peek() {

            if (c.is_ascii_digit() || c == '.') && next.is_alphabetic() {
                result.push('*');
            }
            if c.is_alphabetic() && (next.is_ascii_digit() || next == '.') {
                result.push('*');
            }
            if c == ')' && (next.is_alphanumeric() || next.is_ascii_digit() || next == '(') {
                result.push('*');
            }
            if (c.is_alphabetic() || c.is_ascii_digit()) && next == '(' {
                result.push('*');
            }

            if (!c.is_ascii_digit()) && (next == '.') {
                result.push('0');
            }
          }
      }

      result
  }
