use crate::eval::VarError;

pub fn valid_variable_name(var_name: &String) -> Result<(),VarError> {
    if var_name.is_empty() || !var_name.chars().next().unwrap().is_alphabetic() {
        return Err(VarError::InvalidVariableName(
            format!("Variable name must start with a letter. Got: '{}'", var_name)
        ).into());
    }

    let mut chars = var_name
        .chars()
        .filter(|c| !c.is_whitespace())
        .peekable();

    if var_name.len() != 1 {
        while let Some(_) = chars.next() {
            if let Some(&next) = chars.peek() {
                if !next.is_ascii_digit() {
                    return Err(VarError::InvalidVariableIteration(
                        format!("Variable '{}' must be a single letter or letter followed by digits (e.g., x, x2, y10)", var_name)
                    ).into());
                }
            }
        }
    }
    Ok(())
}
