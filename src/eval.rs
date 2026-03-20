use crate::eval_context::EvalContext;
use crate::unit_conversion::{UNITS_MAGNITUDES, UNITS_KNOWN};
use meval::{Expr};

pub fn evaluate_input(
    eval_ctx: &mut EvalContext,
    init_input: &str,
) -> Option<f64> {
    // Inits input
    let input: String = init_input.to_string();

    // Empty input - nothing to evaluate
    if input.is_empty() {
        return None;
    }


    if input.starts_with("let ") {
        match parse_let_statement(eval_ctx, &input) {
            Some((name, value)) => {
                eval_ctx.ctx.var(&name, value);
                return Some(value);
            }
            None => eprintln!("Invalid let syntax. Use: let x = 5"),
        }
        return None;
    }
    return eval_expr(eval_ctx, &input)

}

fn eval_expr(
    eval_ctx: &mut EvalContext,
    init_input: &str,
) -> Option<f64> {
   let mut input: String = init_input.to_string();

    if input.is_empty() {
        return None;
    }
for n in 1..eval_ctx.counter {
        if let Some(&value) = eval_ctx.parsed_results.get(n - 1) {
            let var_name = format!("lin{}", n);
            eval_ctx.ctx.var(&var_name, value);
        }
    }

    for n in 1..eval_ctx.counter {
        let bare_pattern = format!("lin{}", n);
        let bracketed = format!("[lin{}]", n);

        if input.contains(&bare_pattern) && !input.contains(&bracketed) {
            let mut new_input = String::new();
            let mut remaining = input.as_str();

            while let Some(pos) = remaining.find(&bare_pattern) {
                let after = &remaining[pos + bare_pattern.len()..];
                let is_complete = after.chars().next().map_or(true, |c| !c.is_ascii_digit());

                new_input.push_str(&remaining[..pos]);
                if is_complete {
                    new_input.push_str(&bracketed);
                } else {
                    new_input.push_str(&bare_pattern);
                }
                remaining = after;
            }
            new_input.push_str(remaining);
            input = new_input;
        }
    }

    for (m, x) in UNITS_MAGNITUDES.iter() {
        for v in UNITS_KNOWN.iter() {
            let pattern = format!("{}{}", m, v);
            if input.contains(&pattern) {
                eval_ctx.ctx.var(&pattern, *x);
            }
        }
    }
    let input: &str = &insert_implicit_multiplication(&input);


    match input.parse::<Expr>().and_then(|e| e.eval_with_context(&eval_ctx.ctx)) {
        Ok(result) => {
            eval_ctx.parsed_results.push(result);
            Some(result)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}


fn parse_let_statement(
    eval_ctx: &mut EvalContext,
    input: &str
    ) -> Option<(String, f64)> {
    let rest = input.strip_prefix("let ")?;

    let parts: Vec<&str> = rest.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }
    let var_name = parts[0].trim().to_string();
    let value_str = parts[1].trim();

    let value = eval_expr(eval_ctx, value_str);

    Some((var_name, value?))
}



  fn insert_implicit_multiplication(input: &str) -> String {
      let mut exempt_bracket = false;
      let mut result = String::new();
      let mut chars = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .peekable();

      if chars.peek() == Some(&'.'){
        result.push('0');
        }

      while let Some(c) = chars.next() {

         if c == '[' {
            exempt_bracket = true;
            continue;
          }

         if c ==']' {
            if let Some(&next) = chars.peek(){
                if next.is_alphanumeric() || next == '(' || next == '['{
                result.push('*');
                }
            }
            exempt_bracket = false;
            continue;
        }

        result.push(c);
        if exempt_bracket {
        continue;
        }


          if let Some(&next) = chars.peek() {

            if (c.is_ascii_digit() || c == '.') && next.is_alphabetic() {
                result.push('*');
            }
            if c.is_alphabetic() && (next.is_ascii_digit() || next == '.') {
                result.push('*');
            }
            if c == ')' && (next.is_alphanumeric() || next.is_ascii_digit() || next == '(' || next == '[') {
                result.push('*');
            }
            if (c.is_alphabetic() || c.is_ascii_digit()) && (next == '(' || next == '[') {
                result.push('*');
            }

            if (!c.is_ascii_digit()) && (next == '.') {
                result.push('0');
            }
          }
      }

      result
  }
