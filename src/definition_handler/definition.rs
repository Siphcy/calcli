use crate::eval_context::EvalContext;
use crate::error::{EvalError, DefError};
use super::parse_function::parse_function_definition;
use super::parse_variable::parse_variable_definition;


pub fn assign_batch(eval_ctx: &mut EvalContext, input: &String) -> Result<f64, EvalError> {
    // Clears any previous assigned definitions
    eval_ctx.recently_assigned.clear();

    // Check for mismatched brackets
    let open_brackets = input.matches('[').count();
    let close_brackets = input.matches(']').count();

    if open_brackets != close_brackets {
        if open_brackets > close_brackets {
            return Err(DefError::InvalidDefinitionSyntax(
                format!("Missing {} closing bracket(s) ']'. Use: let [x, y] = [1, 2]", open_brackets - close_brackets)
            ).into());
        } else {
            return Err(DefError::InvalidDefinitionSyntax(
                format!("Missing {} opening bracket(s) '['. Use: let [x, y] = [1, 2]", close_brackets - open_brackets)
            ).into());
        }
    }

    // Check if input has bracket syntax but might be malformed
    if input.contains('[') || input.contains(']') {
        // Check for brackets on both sides
        let parts: Vec<&str> = input.splitn(2, '=').collect();
        if parts.len() == 2 {
            let lhs = parts[0].trim();
            let rhs = parts[1].trim();

            let lhs_has_brackets = lhs.contains('[') && lhs.contains(']');
            let rhs_has_brackets = rhs.contains('[') && rhs.contains(']');

            if lhs_has_brackets && !rhs_has_brackets {
                return Err(DefError::InvalidDefinitionSyntax(
                    "Left side has brackets but right side doesn't. Use: let [x, y] = [1, 2]".to_string()
                ).into());
            }
            if !lhs_has_brackets && rhs_has_brackets {
                return Err(DefError::InvalidDefinitionSyntax(
                    "Right side has brackets but left side doesn't. Use: let [x, y] = [1, 2]".to_string()
                ).into());
            }
        }
    }

    // Matches batch assignments: let [f(x), y, g(z)] = [x^2, 5, z*2]
    if let Some((def_str, values_str)) = parse_batch_assignment(&input) {

        // Check for empty definitions or values
        if def_str.trim().is_empty() {
            return Err(DefError::InvalidDefinitionSyntax(
                "Empty variable list. Use: let [x, y] = [1, 2]".to_string()
            ).into());
        }
        if values_str.trim().is_empty() {
            return Err(DefError::InvalidDefinitionSyntax(
                "Empty value list. Use: let [x, y] = [1, 2]".to_string()
            ).into());
        }

        let defs: Vec<&str> = def_str.split(',').map(|s| s.trim()).collect();
        let values: Vec<&str> = values_str.split(',').map(|s| s.trim()).collect();

        if defs.len() != values.len() {
            return Err(DefError::MismatchedBatch(
                format!("Mismatch: {} variable(s) but {} value(s). Example: let [x, y, z] = [1, 2, 3]",
                    defs.len(), values.len())
            ).into());
        }
        // Assign each definition
        for i in 0..defs.len(){
            assign_definition(eval_ctx, defs[i], values[i])?;

            eval_ctx.recently_assigned.push((defs[i].to_string(), values[i].to_string()));
        }
        return Ok(0.0);
    }

    // If not a batch assignment, handle as single definition
    let (name, var) = tidy_definition_input(input)?;

    eval_ctx.recently_assigned.push((name.to_string(), var.to_string()));
    assign_definition(eval_ctx, name, var)
}

pub fn assign_definition(eval_ctx: &mut EvalContext, def_name: &str, def_value: &str) -> Result<f64, EvalError> {
        if def_name.contains('(') && def_name.contains(')') {
        // Function definition: let f(x) = ...
        let func = parse_function_definition(eval_ctx, def_name, def_value)?;
        let func_name = func.func_name.to_string();

        // Remove and re-insert to move to end of map
        eval_ctx.defined_funcs.shift_remove(&func_name);
        eval_ctx.defined_funcs.insert(func_name, func);
        return Ok(0.0); // Return 0 for function definitions
    } else {
        // Variable definition: let x = ...
        let (name, value) = parse_variable_definition(eval_ctx, def_name, def_value)?;
        eval_ctx.ctx.var(&name, value);

        // Remove and re-insert to move to end of map (handles reassignment)
        eval_ctx.defined_vars.shift_remove(&name);
        eval_ctx.defined_vars.insert(name, value);
        return Ok(value);
    }
}

// Input helper function
fn tidy_definition_input(input: &String) -> Result<(&str, &str), EvalError> {
    let rest = input.strip_prefix("let ").ok_or(DefError::InvalidDefinitionSyntax(
        "Use: let x = 5".to_string()
    ))?;
    let parts: Vec<&str> = rest.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(DefError::InvalidDefinitionSyntax(
            "Missing '=' sign. Use: let x = 5".to_string()
        ).into());
    }
    Ok((parts[0].trim(), parts[1].trim()))
}

/// Parses batch assignment syntax like `let [x, y] = [1, 2]`
/// Returns Some((definitions, values)) if it matches the pattern
fn parse_batch_assignment(input: &str) -> Option<(String, String)> {
    let input = input.trim();

    // Must start with "let "
    let rest = input.strip_prefix("let")?;
    let rest = rest.trim_start();

    // Must start with '['
    if !rest.starts_with('[') {
        return None;
    }

    // Find the first closing bracket
    let first_close = rest.find(']')?;
    let def_str = &rest[1..first_close]; // Extract content between [ and ]

    // After the first ']', there should be '='
    let after_first = rest[first_close + 1..].trim_start();
    if !after_first.starts_with('=') {
        return None;
    }

    // After '=', there should be another '['
    let after_eq = after_first[1..].trim_start();
    if !after_eq.starts_with('[') {
        return None;
    }

    // Find the last ']'
    let last_close = after_eq.rfind(']')?;
    let values_str = &after_eq[1..last_close]; // Extract content between [ and ]

    // Make sure there's nothing after the last ']' except whitespace
    if !after_eq[last_close + 1..].trim().is_empty() {
        return None;
    }

    Some((def_str.to_string(), values_str.to_string()))
}








