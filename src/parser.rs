
use crate::eval_context::EvalContext;
use fancy_regex::Regex;

// Known math functions to ignore during variable substitution
const KNOWN_FUNCTIONS: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
    "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
    "sqrt", "exp", "ln", "log", "log10", "log2",
    "abs", "floor", "ceil", "round",
    "max", "min", "pow"
];

pub fn format_variables(input: String, eval_ctx: &EvalContext) -> String {
    format_variables_with_exclusion(input, eval_ctx, None)
}

pub fn format_variables_with_exclusion(
    mut input: String,
    eval_ctx: &EvalContext,
    exclude_var: Option<&str>
) -> String {
    let line_regex = Regex::new(r"(\[[^\]]*\])|lin(\d+)(?!\d)").unwrap();

    // Format lin# references first
    input = line_regex.replace_all(&input, |caps: &fancy_regex::Captures| {
        if caps.get(1).is_some() {
            caps.get(0).unwrap().as_str().to_string()
        } else {
            format!("[lin{}]", caps.get(2).unwrap().as_str())
        }
    }).to_string();

    let defined_vars = &eval_ctx.defined_vars;

    // Match: anything in brackets, OR single letter + optional digits (not part of a word)
    let var_regex = Regex::new(r"(\[[^\]]*\])|(?<![a-zA-Z])([a-z]\d*)(?![a-zA-Z])").unwrap();

    input = var_regex.replace_all(&input, |caps: &fancy_regex::Captures| {
        if caps.get(1).is_some() {
            // Already in brackets, preserve
            return caps.get(0).unwrap().as_str().to_string();
        }

        let matched = caps.get(2).unwrap().as_str();

        // Check if this variable should be excluded (kept unbbracketed)
        if let Some(excluded) = exclude_var {
            if matched == excluded {
                return format!("[{}]", matched);
            }
        }

        // Check if it's a known function - if so, leave it alone
        if KNOWN_FUNCTIONS.contains(&matched) {
            return matched.to_string();
        }

        // Try to parse as letter + digits
        let mut chars = matched.chars();
        let letter = chars.next().unwrap().to_string();
        let digits: String = chars.collect();

        if !digits.is_empty() {
            // Has digits - prioritize full match (e.g., x2, y10)
            if defined_vars.contains_key(matched) {
                format!("[{}]", matched)
            }
            // Check if just the letter is defined (for cases like x2 meaning x*2)
            else if defined_vars.contains_key(&letter) {
                format!("[{}]{}", letter, digits)
            }
            else {
                matched.to_string()
            }
        } else {
            // No digits - just a single letter
            if defined_vars.contains_key(matched) {
                format!("[{}]", matched)
            } else {
                matched.to_string()
            }
        }
    }).to_string();

    input
}
