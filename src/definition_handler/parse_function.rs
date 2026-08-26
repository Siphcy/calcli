use crate::eval_context::EvalContext;
use crate::error::{EvalError, DefError};
use super::function::Function;
use super::variable::valid_variable_name;
use crate::VARIABLE_SEPARATOR;

pub fn parse_function_definition(
    eval_ctx: &mut EvalContext,
    name: &str,
    value: &str
) -> Result<Function, EvalError> {
    // Check for basic function syntax issues
    if !name.contains('(') {
        return Err(DefError::InvalidDefinitionSyntax(
            format!("Missing '(' in function definition '{}'. Use: let f(x) = expression", name)
        ).into());
    }
    if !name.contains(')') {
        return Err(DefError::InvalidDefinitionSyntax(
            format!("Missing ')' in function definition '{}'. Use: let f(x) = expression", name)
        ).into());
    }

    // Check for empty parentheses
    if name.contains("()") {
        return Err(DefError::InvalidDefinitionSyntax(
            format!("Function '{}' has no parameter. Use: let f(x) = expression", name)
        ).into());
    }

    // Check for multiple parameters (not supported)
    if name.matches(',').count() > 0 {
        return Err(DefError::InvalidDefinitionSyntax(
            format!("Function '{}' has multiple parameters. Only single-parameter functions are supported. Use: let f(x) = expression", name)
        ).into());
    }

    // Parse function name and parameter: f(x) or f_2(x_1)
    let (func_name, var_name) = parse_function_syntax(name)?;

    // Validate both names
    valid_variable_name(&func_name).map_err(|e| {
        DefError::InvalidDefinitionName(
            format!("Invalid function name '{}'. {}", func_name, e)
        )
    })?;

    valid_variable_name(&var_name).map_err(|e| {
        DefError::InvalidDefinitionName(
            format!("Invalid parameter name '{}'. {}", var_name, e)
        )
    })?;

    // Check if function name conflicts with defined variables
    if eval_ctx.defined_vars.contains_key(&func_name) {
        return Err(DefError::InvalidDefinitionName(
            format!("Function name '{}' conflicts with existing variable. Choose a different name.", func_name)
        ).into());
    }


    Ok(Function::new(func_name, var_name, value.to_string()))
}

/// Parses function syntax like f(x) or f_2(x_1)
/// Returns (func_name, var_name) if valid
fn parse_function_syntax(name: &str) -> Result<(String, String), EvalError> {
    let name = name.trim();

    // Find opening and closing parentheses
    let open_paren = name.find('(').ok_or_else(|| {
        DefError::InvalidDefinitionSyntax(
            format!("Invalid function syntax '{}'. Use: let f(x) = expression", name)
        )
    })?;

    let close_paren = name.rfind(')').ok_or_else(|| {
        DefError::InvalidDefinitionSyntax(
            format!("Invalid function syntax '{}'. Use: let f(x) = expression", name)
        )
    })?;

    // Check that close paren is after open paren and at the end
    if close_paren <= open_paren || close_paren != name.len() - 1 {
        return Err(DefError::InvalidDefinitionSyntax(
            format!("Invalid function syntax '{}'. Use: let f(x) = expression", name)
        ).into());
    }

    // Extract function name and parameter
    let func_name = &name[..open_paren];
    let var_name = &name[open_paren + 1..close_paren];

    // Validate function name: lowercase letter optionally followed by separator + digits
    if !is_valid_identifier(func_name) {
        return Err(DefError::InvalidDefinitionSyntax(
            format!("Invalid function format '{}'. Function name must be a single letter optionally followed by '{}' and digits. Use: let f(x) = expression or let f{}2(x{}1) = expression",
                    name, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR)
        ).into());
    }

    // Validate parameter name
    if !is_valid_identifier(var_name) {
        return Err(DefError::InvalidDefinitionSyntax(
            format!("Invalid function format '{}'. Parameter must be a single letter optionally followed by '{}' and digits. Use: let f(x) = expression or let f{}2(x{}1) = expression",
                    name, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR)
        ).into());
    }

    Ok((func_name.to_string(), var_name.to_string()))
}

/// Checks if an identifier matches the pattern: lowercase letter optionally followed by separator + digits
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();

    // First character must be a lowercase letter
    if !chars[0].is_ascii_lowercase() {
        return false;
    }

    // If only one character, it's valid
    if chars.len() == 1 {
        return true;
    }

    // If more than one character, second must be separator
    if chars[1] != VARIABLE_SEPARATOR {
        return false;
    }

    // If we have a separator, we must have at least one digit after it
    if chars.len() == 2 {
        return false;
    }

    // All remaining characters must be alphanumeric
    for i in 2..chars.len() {
        if !chars[i].is_alphanumeric() {
            return false;
        }
    }

    true
}
