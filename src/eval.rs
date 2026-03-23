
use crate::eval_context::EvalContext;
use crate::unit_conversion::{UNITS_MAGNITUDES, UNITS_KNOWN};
use crate::variable::valid_variable_name;
use meval::{Expr};
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
        let (name, value) = parse_let_statement(eval_ctx, &input)?;
        eval_ctx.ctx.var(&name, value);
        eval_ctx.defined_vars.insert(name, value);
        return Ok(value);
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

    for n in 1..eval_ctx.counter {
        if let Some(&value) = eval_ctx.parsed_results.get(n - 1) {
            let var_name = format!("lin{}", n);
            eval_ctx.ctx.var(&var_name, value);
            eval_ctx.defined_vars.insert(var_name, value);
        }
    }

    let line_regex = Regex::new(r"(\[[^\]]*\])|lin(\d+)(?!\d)").unwrap();
    let var_regex = Regex::new(r"(\[[^\]]*\])|([a-z])(\d*)(?!\d)").unwrap();

    input = line_regex.replace_all(&input, |caps: &fancy_regex::Captures| {
        if caps.get(1).is_some() {
            caps.get(0).unwrap().as_str().to_string()
        } else {
            format!("[lin{}]", caps.get(2).unwrap().as_str())
        }
    }).to_string();

    let defined_vars = &eval_ctx.defined_vars;
    input = var_regex.replace_all(&input, |caps: &fancy_regex::Captures| {
        if caps.get(1).is_some() {
            // Already in brackets, preserve
            caps.get(0).unwrap().as_str().to_string()
        } else {
            let letter = caps.get(2).unwrap().as_str();
            let digits = caps.get(3).map_or("", |m| m.as_str());
            let full_var = format!("{}{}", letter, digits);

            // Check if the full variable name exists (like x2, y1, etc.)
            if defined_vars.contains_key(&full_var) {
                format!("[{}]", full_var)
            }
            // Check if just the letter is defined (for cases like x2 meaning x*2)
            else if !digits.is_empty() && defined_vars.contains_key(letter) {
                format!("[{}]{}", letter, digits)
            }
            // Check if just the letter is a variable (without digits)
            else if digits.is_empty() && defined_vars.contains_key(letter) {
                format!("[{}]", letter)
            }
            // Not a defined variable, leave as-is (function name like 'sin', 'ln', etc.)
            else {
                full_var
            }
        }
    }).to_string();
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
