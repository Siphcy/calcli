#[allow(dead_code, unused_imports)]
use crate::eval_context::EvalContext;
use crate::variable::valid_variable_name;
use crate::function::Function;
use meval::{Expr};
use crate::parser::format_variables;
use fancy_regex::Regex;
use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    EmptyInput,
    VarError(VarError),
    ParseError(String),
}

#[derive(Debug)]
pub enum VarError {
    InvalidVariableSyntax(String),
    InvalidVariableName(String),
    InvalidVariableIteration(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::EmptyInput => write!(f, "Empty input"),
            EvalError::VarError(e) => write!(f, "{}", e),
            EvalError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl fmt::Display for VarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarError::InvalidVariableSyntax(msg) => write!(f, "Invalid let syntax: {}", msg),
            VarError::InvalidVariableName(msg) => write!(f, "Invalid variable name: {}", msg),
            VarError::InvalidVariableIteration(msg) => write!(f, "Invalid variable iteration: {}", msg),
        }
    }
}

impl std::error::Error for EvalError {}
impl std::error::Error for VarError {}

impl From<VarError> for EvalError {
    fn from(e: VarError) -> Self {
        EvalError::VarError(e)
    }
}

pub fn evaluate_input(
    eval_ctx: &mut EvalContext,
    init_input: &str,
) -> Result<f64, EvalError> {
    // Inits input
    let input: String = init_input.to_string();

    // Empty input - nothing to evaluate
    if input.is_empty() {
        return Err(EvalError::EmptyInput);
    }

    if input.starts_with("let ") {
        // Check if it's a function definition (contains parentheses)
        if input.contains('(') && input.contains(')') {
            let func = parse_function_definition(eval_ctx, &input)?;
            let func_name = func.func_name.clone();
            eval_ctx.defined_funcs.insert(func_name, func);
            return Ok(0.0); // Return 0 for function definitions
        } else {
            // Variable definition
            let (name, value) = parse_let_statement(eval_ctx, &input)?;
            eval_ctx.ctx.var(&name, value);
            eval_ctx.defined_vars.insert(name, value);
            return Ok(value);
        }
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

    for n in 1..eval_ctx.counter {
        if let Some(&value) = eval_ctx.parsed_results.get(n - 1) {
            let var_name = format!("lin{}", n);
            eval_ctx.ctx.var(&var_name, value);
            eval_ctx.defined_vars.insert(var_name, value);
        }
    }

    // Evaluate function calls before processing variables
    input = evaluate_function_calls(eval_ctx, &input)?;

    let input = format_variables(input, eval_ctx);


    let input: &str = &insert_implicit_multiplication(&input);

    // Convert brackets to parentheses for meval (which only supports parentheses)
    let input = input.replace('[', "(").replace(']', ")");

    match input.parse::<Expr>().and_then(|e| e.eval_with_context(&eval_ctx.ctx)) {
        Ok(result) => {
            eval_ctx.parsed_results.push(result);
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

fn parse_function_definition(
    eval_ctx: &mut EvalContext,
    input: &str
) -> Result<Function, EvalError> {
    let rest = input.strip_prefix("let ").ok_or(VarError::InvalidVariableSyntax(
        "Use: let f(x) = expression".to_string()
    ))?;

    // Split by '=' to get left (func definition) and right (expression)
    let parts: Vec<&str> = rest.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(VarError::InvalidVariableSyntax(
            "Missing '=' sign. Use: let f(x) = expression".to_string()
        ).into());
    }

    let left = parts[0].trim();
    let expr = parts[1].trim().to_string();

    // Parse function name and parameter: f(x) or f2(x1)
    let func_regex = Regex::new(r"^([a-z]\d*)\(([a-z]\d*)\)$").unwrap();
    let caps = func_regex.captures(left)
        .map_err(|_| VarError::InvalidVariableSyntax(
            "Invalid function definition syntax".to_string()
        ))?
        .ok_or(VarError::InvalidVariableSyntax(
            "Use: let f(x) = expression".to_string()
        ))?;

    let func_name = caps.get(1).unwrap().as_str().to_string();
    let var_name = caps.get(2).unwrap().as_str().to_string();

    // Validate both names
    valid_variable_name(&func_name)?;
    valid_variable_name(&var_name)?;

    // Check if function name conflicts with defined variables
    if eval_ctx.defined_vars.contains_key(&func_name) {
        return Err(VarError::InvalidVariableName(
            format!("Function name '{}' conflicts with existing variable", func_name)
        ).into());
    }

    Ok(Function::new(func_name, var_name, expr))
}

fn parse_let_statement(
    eval_ctx: &mut EvalContext,
    input: &str
) -> Result<(String, f64), EvalError> {
    let rest = input.strip_prefix("let ").ok_or(VarError::InvalidVariableSyntax(
        "Use: let x = 5".to_string()
    ))?;

    let parts: Vec<&str> = rest.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(VarError::InvalidVariableSyntax(
            "Missing '=' sign. Use: let x = 5".to_string()
        ).into());
    }

    let var_name = parts[0].trim().to_string();
    valid_variable_name(&var_name)?;


    let value_str = parts[1].trim();
    let value = eval_expr(eval_ctx, value_str)?;

    Ok((var_name, value))
}

//TODO: Finish Convertor Parser
/*
fn parse_conv_statement(
    eval_ctx: &mut EvalContext,
    input: &str
    ) -> Result<(String, f64), Error> {
for (m, x) in UNITS_MAGNITUDES.iter() {
        for v in UNITS_KNOWN.iter() {
            let pattern = format!("{}{}", m, v);
            if input.contains(&pattern) {
                eval_ctx.ctx.var(&pattern, *x);
            }
        }
    }



}*/



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
