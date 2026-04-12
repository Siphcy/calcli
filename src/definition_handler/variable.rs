use crate::error::DefError;
use crate::VARIABLE_SEPARATOR;

pub fn valid_variable_name(var_name: &String) -> Result<(),DefError> {
    if var_name.is_empty() || !var_name.chars().next().unwrap().is_alphabetic() {
        return Err(DefError::InvalidDefinitionName(
            format!("Variable name must start with a letter. Got: '{}'", var_name)
        ).into());
    }

    let mut chars = var_name
        .chars()
        .filter(|c| !c.is_whitespace())
        .peekable();

    // Single letter is valid (count characters, not bytes for Unicode support)
    if var_name.chars().count() == 1 {
        return Ok(());
    }

    // Skip first character (already validated as alphabetic)
    chars.next();

    // Check if second character is the separator
    if let Some(&second_char) = chars.peek() {
        if second_char == VARIABLE_SEPARATOR {
            // Consume the separator
            chars.next();

            // Must have at least one digit after separator
            if chars.peek().is_none() {
                return Err(DefError::InvalidDefinitionIteration(
                    format!("Variable '{}' must be a single letter or letter followed by '{}' and digits (e.g., x, x{}2, y{}10)",
                            var_name, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR)
                ).into());
            }

            // All remaining characters must be digits
            while let Some(&ch) = chars.peek() {
                if !ch.is_alphanumeric() {
                    return Err(DefError::InvalidDefinitionIteration(
                        format!("Variable '{}' must be a single letter or letter followed by '{}' and alphanumeric characters (e.g., x, x{}2, y{}10)",
                                var_name, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR)
                    ).into());
                }
                chars.next();
            }
        } else {
            // Second character is not separator - invalid format
            return Err(DefError::InvalidDefinitionIteration(
                format!("Variable '{}' must be a single letter or letter followed by '{}' and digits (e.g., x, x{}2, y{}10)",
                        var_name, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR, VARIABLE_SEPARATOR)
            ).into());
        }
    }

    Ok(())
}


