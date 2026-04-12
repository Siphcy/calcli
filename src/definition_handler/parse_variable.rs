use crate::eval_context::EvalContext;
use crate::eval::evaluate_input;
use crate::error::{EvalError, DefError};
use super::variable::valid_variable_name;

pub fn parse_variable_definition(
    eval_ctx: &mut EvalContext,
    name: &str, value: &str
) -> Result<(String, f64), EvalError> {
    // Check for empty variable name
    if name.trim().is_empty() {
        return Err(DefError::InvalidDefinitionSyntax(
            "Variable name cannot be empty. Use: let x = 5".to_string()
        ).into());
    }

    // Check for conflicts with existing functions
    if eval_ctx.defined_funcs.contains_key(name) {
        return Err(DefError::InvalidDefinitionName(
            format!("Variable name '{}' conflicts with existing function. Choose a different name.", name)
        ).into());
    }

    // Validate variable name format
    valid_variable_name(&name.to_string()).map_err(|e| {
        DefError::InvalidDefinitionName(
            format!("Invalid variable name '{}'. {}", name, e)
        )
    })?;

    // Check for empty value
    if value.trim().is_empty() {
        return Err(DefError::InvalidDefinitionSyntax(
            format!("Variable '{}' has no value. Use: let {} = 5", name, name)
        ).into());
    }

    // Evaluate the value expression
    let parsed_value = evaluate_input(eval_ctx, &value, true).map_err(|e| {
        DefError::InvalidDefinitionSyntax(
            format!("Cannot evaluate value for '{}': {}", name, e)
        )
    })?;

    Ok((name.to_string(), parsed_value))
}
