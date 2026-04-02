use crate::eval_context::EvalContext;
use crate::error::{EvalError, DefError};
use super::function::Function;
use super::variable::valid_variable_name;
use fancy_regex::Regex;

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

    // Parse function name and parameter: f(x) or f2(x1)
    let func_regex = Regex::new(r"^([a-z]\d*)\(([a-z]\d*)\)$").unwrap();
    let caps = func_regex.captures(name)
        .map_err(|_| DefError::InvalidDefinitionSyntax(
            format!("Invalid function syntax '{}'. Use: let f(x) = expression", name)
        ))?
        .ok_or(DefError::InvalidDefinitionSyntax(
            format!("Invalid function format '{}'. Function name and parameter must be single letters optionally followed by digits. Use: let f(x) = expression", name)
        ))?;

    let func_name = caps.get(1).unwrap().as_str().to_string();
    let var_name = caps.get(2).unwrap().as_str().to_string();

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
