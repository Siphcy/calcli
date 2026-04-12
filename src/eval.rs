use crate::eval_context::EvalContext;
use crate::definition_handler::definition::assign_batch;
use crate::error::EvalError;
use meval::Expr;
use crate::parser::format_variables;
use crate::conversion_handler::scientific_notation::convert_to_scientific;
use crate::VARIABLE_SEPARATOR;
use crate::implicit_multiplication::insert_implicit_multiplication;

//TODO: Finish Convertor Parser

pub fn evaluate_input(
    eval_ctx: &mut EvalContext,
    init_input: &str,
    evaluate_new_line: bool,
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

    // Handle 'precision' command
    if input.starts_with("precision ") {
        let value_str = input.strip_prefix("precision ").unwrap().trim();
        match value_str.parse::<usize>() {
            Ok(n) if n > 0 && n <= 15 => {
                eval_ctx.precision = n;
                return Err(EvalError::ParseError(
                    format!("Precision set to {} significant figures", n)
                ));
            }
            Ok(_) => {
                return Err(EvalError::ParseError(
                    "Precision must be between 1 and 15".to_string()
                ));
            }
            Err(_) => {
                return Err(EvalError::ParseError(
                    "Usage: precision <number>  (e.g., precision 6)".to_string()
                ));
            }
        }
    }

    // Handle 'sci' command
    if input.starts_with("sci ") || input == "sci toggle" {
        if input == "sci toggle" {
            eval_ctx.sci_notation_enabled = !eval_ctx.sci_notation_enabled;
            let status = if eval_ctx.sci_notation_enabled { "enabled" } else { "disabled" };
            return Err(EvalError::ParseError(
                format!("Scientific notation {}", status)
            ));
        }

        // Extract the argument after "sci "
        let arg = input.strip_prefix("sci ").unwrap().trim();

        // Try to parse as a direct number first
        if let Ok(num) = arg.parse::<f64>() {
            let result = convert_to_scientific(num, eval_ctx.precision);
            return Err(EvalError::ParseError(result));
        }

        // Try to evaluate as an expression (e.g., "sci lin1" or "sci 2+3")
        match eval_expr(eval_ctx, arg, evaluate_new_line) {
            Ok(num) => {
                let result = convert_to_scientific(num, eval_ctx.precision);
                return Err(EvalError::ParseError(result));
            }
            Err(_) => {
                return Err(EvalError::ParseError(
                    "Usage: sci <number>, sci <expression>, or sci toggle".to_string()
                ));

            }
        }
    }

    // Detect bare assignment syntax like `x=5` or `x = 5`
    if let Some((var_name, expr)) = parse_bare_assignment(&input) {
        return Err(EvalError::ParseError(
            format!("Did you mean: `let {} = {}`?", var_name, expr)
        ));
    }

    eval_expr(eval_ctx, &input, evaluate_new_line)
}

fn eval_expr(
    eval_ctx: &mut EvalContext,
    init_input: &str,
    evaluate_new_line: bool,
) -> Result<f64, EvalError> {
    let mut input: String = init_input.to_string();

    if input.is_empty() {
        return Err(EvalError::EmptyInput);
    }

    input = format_variables(input, eval_ctx);
    // Evaluate function calls before processing variables
    input = evaluate_function_calls(eval_ctx, &input)?;



    let input: &str = &insert_implicit_multiplication(&input);

    // Substitute non-ASCII variables with their values (meval doesn't support Unicode var names)
    let input = substitute_non_ascii_vars(&input, eval_ctx);

    // Convert brackets to parentheses for meval (which only supports parentheses)
    let input = input.replace('[', "(").replace(']', ")");

    match input.parse::<Expr>().and_then(|e| e.eval_with_context(&eval_ctx.ctx)) {
        Ok(result) => {
        if evaluate_new_line == true {
            eval_ctx.counter += 1;
            let line_name = format!("lin{}{}",VARIABLE_SEPARATOR, eval_ctx.counter);
            eval_ctx.ctx.var(&line_name, result);
            eval_ctx.defined_vars.insert(line_name, result);
            }
            Ok(result)
        }
        Err(e) => {
            Err(EvalError::ParseError(e.to_string()))
        }
    }
}

/// Parses bare assignment syntax like `x=5` or `var123 = expression`
/// Returns Some((var_name, expression)) if it matches the pattern
fn parse_bare_assignment(input: &str) -> Option<(String, String)> {
    let input = input.trim();

    // Find the '=' sign
    let eq_pos = input.find('=')?;

    // Extract parts
    let lhs = input[..eq_pos].trim();
    let rhs = input[eq_pos + 1..].trim();

    // Check if rhs is empty
    if rhs.is_empty() {
        return None;
    }

    // Check if lhs matches the pattern: starts with letter, followed by letters/digits
    if lhs.is_empty() {
        return None;
    }

    let mut chars = lhs.chars();
    let first = chars.next()?;

    if first.is_ascii_digit() {
        return None;
    }

    for ch in chars {
        if !ch.is_ascii_digit() {
            return None;
        }
    }

    Some((lhs.to_string(), rhs.to_string()))
}

fn evaluate_function_calls(
    eval_ctx: &mut EvalContext,
    input: &str,
) -> Result<String, EvalError> {
    let mut result = input.to_string();

    // Keep replacing function calls until there are none left
    // This handles nested calls like f(g(5))
    loop {
        // Find a function call: func_name(args) where func_name can be like f or f_2
        let func_call = find_function_call(&result)?;

        match func_call {
            None => break, // No more function calls found
            Some((full_match_start, full_match_end, func_name, arg)) => {
                // Check if this function is defined
                if let Some(func) = eval_ctx.defined_funcs.get(&func_name).cloned() {
                    // Evaluate the function with the argument
                    let func_result = func.evaluate_func(eval_ctx, &arg)?;

                    // Replace the function call with the result
                    let replacement = func_result.to_string();
                    result.replace_range(full_match_start..full_match_end, &replacement);
                } else {
                    // Not a user-defined function, could be a built-in like sin, cos, etc.
                    // Leave it as-is and continue
                    break;
                }
            }
        }
    }

    Ok(result)
}

/// Finds a function call in the input string
/// Returns Some((start, end, func_name, arg)) or None
fn find_function_call(input: &str) -> Result<Option<(usize, usize, String, String)>, EvalError> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' && Some(chars[i+1].is_ascii() || !chars[i+1].is_ascii_digit()).is_some() {
            i += 1;
            let func_start = i;

            let mut func_name = String::new();
            func_name.push(chars[i]);
            i += 1;

            // Check for optional separator + digits (e.g., f_2)
            if i < chars.len() && chars[i] == VARIABLE_SEPARATOR {
                func_name.push(chars[i]);
                i += 1;

                // Collect subscript
                let subscript_start = i;
                while i < chars.len() && chars[i].is_ascii() {
                    func_name.push(chars[i]);
                    i += 1;
                }

                // If no subscript after separator, it's not a valid function name
                if i == subscript_start {
                    continue;
                }
            }

            // Check if followed by '('
            if i < chars.len() && chars[i] == '(' {
                i += 1; // Skip '('

                // Find the matching ')'
                let arg_start = i;
                let mut paren_count = 1;
                let mut found_close = false;

                while i < chars.len() {
                    if chars[i] == '(' {
                        paren_count += 1;
                    } else if chars[i] == ')' {
                        paren_count -= 1;
                        if paren_count == 0 {
                            found_close = true;
                            break;
                        }
                    }
                    i += 1;
                }

                if found_close {
                    let arg: String = chars[arg_start..i].iter().collect();

                    // Convert character indices to byte indices for UTF-8 safety
                    let byte_start = chars[..func_start-1].iter().map(|c| c.len_utf8()).sum();
                    let byte_end = chars[..=i+1].iter().map(|c| c.len_utf8()).sum();

                    return Ok(Some((byte_start, byte_end, func_name, arg)));
                }
            }
        } else {
            i += 1;
        }
    }

    Ok(None)
}

/// Substitutes non-ASCII variables with their values
/// meval doesn't support Unicode variable names, so we pre-substitute them
fn substitute_non_ascii_vars(input: &str, eval_ctx: &EvalContext) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '[' {
            // Found a bracket, need to match it properly (handle nested brackets)
            let _bracket_start = i;
            i += 1;
            let var_start = i;

            // Track bracket depth to handle nested brackets like [sin([θ])]
            let mut bracket_depth = 1;
            while i < chars.len() && bracket_depth > 0 {
                if chars[i] == '[' {
                    bracket_depth += 1;
                } else if chars[i] == ']' {
                    bracket_depth -= 1;
                }
                if bracket_depth > 0 {
                    i += 1;
                }
            }

            if i < chars.len() && chars[i] == ']' {
                let var_name: String = chars[var_start..i].iter().collect();

                // Check if this is a simple variable (no nested brackets or other complex expressions)
                let is_simple_var = !var_name.contains('[') && !var_name.contains('(');

                // Check if variable name contains non-ASCII characters and is a simple variable
                if is_simple_var && var_name.chars().any(|c| !c.is_ascii()) {
                    // Look up the value
                    if let Some(&value) = eval_ctx.defined_vars.get(&var_name) {
                        // Substitute with the value
                        result.push_str(&value.to_string());
                        i += 1; // Skip the closing bracket
                        continue;
                    }
                }

                // Complex expression or ASCII variable - recursively process the content
                let processed_content = substitute_non_ascii_vars(&var_name, eval_ctx);
                result.push('[');
                result.push_str(&processed_content);
                result.push(']');
                i += 1;
            } else {
                // No closing bracket found, keep the opening bracket
                result.push('[');
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}


