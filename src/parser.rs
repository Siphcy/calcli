
use crate::eval_context::EvalContext;
use fancy_regex::Regex;
use crate::{VARIABLE_SEPARATOR, escape_separator};

// Known math functions to ignore during variable substitution
const KNOWN_FUNCTIONS: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
    "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
    "sqrt", "exp", "ln", "log", "log10", "log2",
    "abs", "floor", "ceil", "round",
    "max", "min", "pow",
];

pub fn format_variables(input: String, eval_ctx: &EvalContext) -> String {
    format_variables_with_exclusion(input, eval_ctx, None)
}

pub fn format_variables_with_exclusion(
    mut input: String,
    eval_ctx: &EvalContext,
    exclude_var: Option<&str>
) -> String {
    // Dynamically build regex patterns with separator
    let sep_escaped = escape_separator();
    let line_pattern = format!(r"(\[[^\]]*\])|lin{}(\d+)(?!\d)", sep_escaped);
    let line_regex = Regex::new(&line_pattern).unwrap();
    // Match sequences of letters (with optional trailing separator and digits) to check for functions first
    let word_pattern = format!(r"(\[[^\]]*\])|(?<![a-zA-Z])([a-z]+)(?:{}(\d+))?", sep_escaped);
    let word_regex = Regex::new(&word_pattern).unwrap();

    input = line_regex.replace_all(&input, |caps: &fancy_regex::Captures| {
        if caps.get(1).is_some() {
            caps.get(0).unwrap().as_str().to_string()
        } else {
            format!("[lin{}{}]", VARIABLE_SEPARATOR, caps.get(2).unwrap().as_str())
        }
    }).to_string();

    let defined_vars = &eval_ctx.defined_vars;
    input = word_regex.replace_all(&input, |caps: &fancy_regex::Captures| {
        if caps.get(1).is_some() {
            // Already in brackets, preserve
            return caps.get(0).unwrap().as_str().to_string();
        }

        let letters = caps.get(2).unwrap().as_str();
        let digits = caps.get(3).map_or("", |m| m.as_str());
        let full_match = if !digits.is_empty() {
            format!("{}{}{}", letters, VARIABLE_SEPARATOR, digits)
        } else {
            letters.to_string()
        };

        // Check if this is the excluded variable (function parameter) - always bracket it
        if let Some(excluded) = exclude_var {
            if full_match == excluded {
                return format!("[{}]", full_match);
            }
        }

        // Check if it's a known function - leave it alone
        if KNOWN_FUNCTIONS.contains(&letters) {
            return full_match;
        }

        // Check if the entire sequence is a defined variable (like x_2, y_1, abc, etc.)
        if defined_vars.contains_key(&full_match) {
            return format!("[{}]", full_match);
        }

        // For single letter (with or without digits after separator)
        if letters.len() == 1 {
            if digits.is_empty() && defined_vars.contains_key(letters) {
                // Single letter variable like 'x'
                return format!("[{}]", letters);
            }
            // If digits are present (e.g., x_2) but not defined, leave as-is
            // This will likely cause an error later, which is desired behavior
            return full_match;
        }

        // Multiple letters - try to split into known variables/constants
        // Special case: if we have digits, check if suffix + digits forms a variable
        let mut result = String::new();
        let mut remaining = letters;

        // If we have digits, try to find a split where the last part + digits is a variable
        if !digits.is_empty() {
            // Try different split points
            for split_pos in 0..letters.len() {
                let prefix = &letters[0..split_pos];
                let suffix_with_digits = format!("{}{}{}", &letters[split_pos..], VARIABLE_SEPARATOR, digits);

                if defined_vars.contains_key(&suffix_with_digits) {
                    // Found a valid split! Process the prefix part
                    remaining = prefix;

                    while !remaining.is_empty() {
                        let mut matched = false;
                        for len in (1..=remaining.len()).rev() {
                            let candidate = &remaining[0..len];
                            if defined_vars.contains_key(candidate) {
                                result.push_str(&format!("[{}]", candidate));
                                remaining = &remaining[len..];
                                matched = true;
                                break;
                            }
                        }
                        if !matched {
                            result.push(remaining.chars().next().unwrap());
                            remaining = &remaining[1..];
                        }
                    }

                    // Append the suffix_with_digits part
                    result.push_str(&format!("[{}]", suffix_with_digits));
                    return result;
                }
            }
        }

        // Normal processing - no special digit handling needed
        while !remaining.is_empty() {
            let mut matched = false;

            // Try matching longest possible variable name first (up to remaining length)
            // Check lengths from longest to shortest to prioritize multi-char constants like "pi"
            for len in (1..=remaining.len()).rev() {
                let candidate = &remaining[0..len];
                if defined_vars.contains_key(candidate) {
                    result.push_str(&format!("[{}]", candidate));
                    remaining = &remaining[len..];
                    matched = true;
                    break;
                }
            }

            // If no match found, take first character as-is
            if !matched {
                result.push(remaining.chars().next().unwrap());
                remaining = &remaining[1..];
            }
        }

        // Append digits with separator if present (only if we didn't find a suffix match above)
        if !digits.is_empty() {
            result.push_str(&format!("{}{}", VARIABLE_SEPARATOR, digits));
        }
        result
    }).to_string();

    input
}
