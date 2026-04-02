use crate::eval_context::EvalContext;
use crate::definition_handler::definition::assign_batch;
use crate::error::EvalError;
use meval::Expr;
use crate::parser::format_variables;
use fancy_regex::Regex;


//TODO: Finish Convertor Parser

pub fn evaluate_input(
    eval_ctx: &mut EvalContext,
    init_input: &str,
) -> Result<f64, EvalError> {
    // Inits input
    let input: String = init_input.trim().to_string();

    // Empty input - nothing to evaluate
    if input.is_empty() {
        return Err(EvalError::EmptyInput);
    }

    if input.starts_with("let ") {
        // assigns definitions
        return assign_batch(eval_ctx, &input);
    }

    // Handle remove/delete commands: remove x, delete f, rm y
    if input.starts_with("remove ") || input.starts_with("delete ") || input.starts_with("rm ") {
        let name = input
            .strip_prefix("remove ")
            .or_else(|| input.strip_prefix("delete "))
            .or_else(|| input.strip_prefix("rm "))
            .unwrap()
            .trim();

        if name.is_empty() {
            return Err(EvalError::ParseError(
                "Usage: remove <name>  (e.g., remove x, rm f)".to_string()
            ));
        }

        let mut found = false;

        // Try to remove as variable
        if eval_ctx.defined_vars.shift_remove(name).is_some() {
            found = true;
            // Rebuild meval context without this variable
            eval_ctx.ctx = meval::Context::new();
            for (var_name, var_value) in &eval_ctx.defined_vars {
                eval_ctx.ctx.var(var_name, *var_value);
            }
        }

        // Try to remove as function
        if eval_ctx.defined_funcs.shift_remove(name).is_some() {
            found = true;
        }

        if found {
            return Ok(0.0);
        }

        // Not found
        return Err(EvalError::ParseError(
            format!("'{}' not found. Cannot remove undefined variable or function.", name)
        ));
    }

    // Detect bare assignment syntax like `x=5` or `x = 5`
    let assign_regex = Regex::new(r"^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(.+)$").unwrap();
    if let Ok(Some(caps)) = assign_regex.captures(&input) {
        let var_name = caps.get(1).unwrap().as_str();
        let expr = caps.get(2).unwrap().as_str();
        return Err(EvalError::ParseError(
            format!("Did you mean: `let {} = {}`?", var_name, expr)
        ));
    }

    eval_expr(eval_ctx, &input)
}

fn eval_expr(
    eval_ctx: &mut EvalContext,
    init_input: &str,
) -> Result<f64, EvalError> {
    let mut input: String = init_input.to_string();

    if input.is_empty() {
        return Err(EvalError::EmptyInput);
    }

    // Evaluate function calls before processing variables
    input = evaluate_function_calls(eval_ctx, &input)?;

    let input = format_variables(input, eval_ctx);


    let input: &str = &insert_implicit_multiplication(&input);

    // Convert brackets to parentheses for meval (which only supports parentheses)
    let input = input.replace('[', "(").replace(']', ")");

    match input.parse::<Expr>().and_then(|e| e.eval_with_context(&eval_ctx.ctx)) {
        Ok(result) => {

            eval_ctx.counter += 1;
            let line_name = format!("lin{}",eval_ctx.counter);
            eval_ctx.ctx.var(&line_name, result);
            eval_ctx.defined_vars.insert(line_name, result);

            Ok(result)
        }
        Err(e) => {
            Err(EvalError::ParseError(e.to_string()))
        }
    }
}

fn evaluate_function_calls(
    eval_ctx: &mut EvalContext,
    input: &str,
) -> Result<String, EvalError> {
    let mut result = input.to_string();

    // Regex to match function calls: f(expression) or f2(expression)
    // This matches function_name(anything_inside_parens)
    let func_call_regex = Regex::new(r"([a-z]\d*)\(([^()]+)\)").unwrap();

    // Keep replacing function calls until there are none left
    // This handles nested calls like f(g(5))
    loop {
        let caps = match func_call_regex.captures(&result) {
            Ok(Some(caps)) => caps,
            Ok(None) => break, // No more function calls found
            Err(_) => break,
        };

        let full_match = caps.get(0).unwrap().as_str();
        let func_name = caps.get(1).unwrap().as_str();
        let arg = caps.get(2).unwrap().as_str();

        // Check if this function is defined
        if let Some(func) = eval_ctx.defined_funcs.get(func_name).cloned() {
            // Evaluate the function with the argument
            let func_result = func.evaluate_func(eval_ctx, arg)?;

            // Replace the function call with the result
            result = result.replace(full_match, &func_result.to_string());
        } else {
            // Not a user-defined function, could be a built-in like sin, cos, etc.
            // Leave it as-is and continue
            break;
        }
    }

    Ok(result)
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
            // Add implicit multiplication before '[' if needed
            if let Some(last) = result.chars().last() {
                if last.is_ascii_digit() || last == ')' {
                    result.push('*');
                }
            }
            result.push('[');
            exempt_bracket = true;
            continue;
          }

         if c ==']' {
            result.push(']');
            exempt_bracket = false;
            // Add implicit multiplication after ']' if needed
            if let Some(&next) = chars.peek(){
                if next.is_alphanumeric() || next == '(' || next == '['{
                result.push('*');
                }
            }
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

            if c.is_ascii_digit() && (next == '(' || next == '[') {
                result.push('*');
            }

            if c == ')' && (next.is_alphanumeric() || next.is_ascii_digit() || next == '(' || next == '[') {
                result.push('*');
            }

            if (!c.is_ascii_digit()) && (next == '.') {
                result.push('0');
            }
          }
      }

      result
  }
