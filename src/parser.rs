
use crate::eval_context::EvalContext;
use crate::VARIABLE_SEPARATOR;

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
    // First pass: Replace lin_<digits> patterns with [lin_<digits>]
    input = process_line_patterns(input);

    // Second pass: Process word patterns (letter sequences with optional subscripts)
    input = process_word_patterns(input, eval_ctx, exclude_var);

    input
}

/// Process patterns like lin_123 (but not in brackets) and wrap them in brackets
fn process_line_patterns(input: String) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Skip content already in brackets
        if chars[i] == '[' {
            let _bracket_start = i;
            result.push(chars[i]);
            i += 1;

            // Find the closing bracket
            while i < chars.len() {
                result.push(chars[i]);
                if chars[i] == ']' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Check for "lin" pattern
        if i + 3 < chars.len() &&
           chars[i] == 'l' && chars[i+1] == 'i' && chars[i+2] == 'n' &&
           chars[i+3] == VARIABLE_SEPARATOR {
            // Found "lin_", now check if followed by digits
            let mut j = i + 4;
            while j < chars.len() && chars[j].is_ascii_digit() {
                j += 1;
            }

            if j > i + 4 {
                // We have lin_<digits>
                result.push_str("[lin");
                result.push(VARIABLE_SEPARATOR);
                for k in (i+4)..j {
                    result.push(chars[k]);
                }
                result.push(']');
                i = j;
                continue;
            }
        }

        // Not a special pattern, just copy the character
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Process word patterns (sequences of lowercase letters with optional subscripts)
fn process_word_patterns(
    input: String,
    eval_ctx: &EvalContext,
    exclude_var: Option<&str>
) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let defined_vars = &eval_ctx.defined_vars;

    while i < chars.len() {
        // Skip content already in brackets
        if chars[i] == '[' {
            let _bracket_start = i;
            result.push(chars[i]);
            i += 1;

            // Find the closing bracket
            while i < chars.len() {
                result.push(chars[i]);
                if chars[i] == ']' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Check if we're at the start of a word (not preceded by a letter)
        let prev_is_letter = if i > 0 {
            chars[i-1].is_ascii_alphabetic()
        } else {
            false
        };

        // If current character is a lowercase letter and not preceded by a letter
        if chars[i].is_ascii_lowercase() && !prev_is_letter {
            // Collect all lowercase letters
            let letter_start = i;
            while i < chars.len() && chars[i].is_ascii_lowercase() {
                i += 1;
            }
            let letters: String = chars[letter_start..i].iter().collect();

            // Check if followed by separator and digits
            let mut digits = String::new();
            if i < chars.len() && chars[i] == VARIABLE_SEPARATOR {
                let sep_pos = i;
                i += 1; // Skip separator

                // Collect digits
                while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                    digits.push(chars[i]);
                    i += 1;
                }

                // If no digits found after separator, backtrack
                if digits.is_empty() {
                    i = sep_pos;
                }
            }

            // Build the full match (letters + optional separator + digits)
            let full_match = if !digits.is_empty() {
                format!("{}{}{}", letters, VARIABLE_SEPARATOR, digits)
            } else {
                letters.clone()
            };

            // Apply transformation logic
            let transformed = process_variable_match(&letters, &digits, &full_match, defined_vars, exclude_var);
            result.push_str(&transformed);
        } else {
            // Not a lowercase letter at word boundary, just copy
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Process a matched variable pattern and decide how to transform it
fn process_variable_match(
    letters: &str,
    digits: &str,
    full_match: &str,
    defined_vars: &indexmap::IndexMap<String, f64>,
    exclude_var: Option<&str>
) -> String {
    // Check if this is the excluded variable (function parameter) - always bracket it
    if let Some(excluded) = exclude_var {
        if full_match == excluded {
            return format!("[{}]", full_match);
        }
    }

    // Check if it's a known function - leave it alone
    if KNOWN_FUNCTIONS.contains(&letters) {
        return full_match.to_string();
    }

    // Check if the entire sequence is a defined variable (like x_2, y_1, abc, etc.)
    if defined_vars.contains_key(full_match) {
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
        return full_match.to_string();
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
}
