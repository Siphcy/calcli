
use crate::eval_context::EvalContext;
use fancy_regex::Regex;

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
    let line_regex = Regex::new(r"(\[[^\]]*\])|lin(\d+)(?!\d)").unwrap();
    // Match sequences of letters (with optional trailing digits) to check for functions first
    let word_regex = Regex::new(r"(\[[^\]]*\])|(?<![a-zA-Z])([a-z]+)(\d*)").unwrap();

    input = line_regex.replace_all(&input, |caps: &fancy_regex::Captures| {
        if caps.get(1).is_some() {
            caps.get(0).unwrap().as_str().to_string()
        } else {
            format!("[lin{}]", caps.get(2).unwrap().as_str())
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
        let full_match = format!("{}{}", letters, digits);

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

        // Check if the entire sequence is a defined variable (like x2, y1, abc, etc.)
        if defined_vars.contains_key(&full_match) {
            return format!("[{}]", full_match);
        }

        // For single letter with digits (like x2)
        if letters.len() == 1 {
            if !digits.is_empty() && defined_vars.contains_key(letters) {
                // x2 where x is defined -> [x]2
                return format!("[{}]{}", letters, digits);
            } else if digits.is_empty() && defined_vars.contains_key(letters) {
                // Single letter variable
                return format!("[{}]", letters);
            }
            return full_match;
        }

        // Multiple letters - split into individual variables
        let mut result = String::new();
        for ch in letters.chars() {
            let ch_str = ch.to_string();
            if defined_vars.contains_key(&ch_str) {
                result.push_str(&format!("[{}]", ch_str));
            } else {
                result.push(ch);
            }
        }
        result.push_str(digits);
        result
    }).to_string();

    input
}
