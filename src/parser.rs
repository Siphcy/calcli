
use crate::eval_context::EvalContext;
use crate::VARIABLE_SEPARATOR;
use crate::constant::KNOWN_FUNCTIONS;



pub fn format_variables(input: String, eval_ctx: &EvalContext) -> String {
    format_variables_with_exclusion(input, eval_ctx, None)
}

pub fn format_variables_with_exclusion(
    mut input: String,
    eval_ctx: &EvalContext,
    exclude_var: Option<&str>
) -> String {
    input = process_line_patterns(input);

    input = process_definition_patterns(input, eval_ctx, exclude_var);

    input
}

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
            // Found "lin_", now check if followed by subscript
            let mut j = i + 4;
            while j < chars.len() && chars[j].is_ascii_digit() {
                j += 1;
            }

            if j > i + 4 {
                // We have lin_<subscript>
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

fn process_definition_patterns(
    input: String,
    eval_ctx: &EvalContext,
    exclude_var: Option<&str>
) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let defined_vars = &eval_ctx.defined_vars;
    let defined_funcs = &eval_ctx.defined_funcs;

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

        if chars[i].is_alphabetic() {
            // Check if we're at a word boundary (for multi-letter pattern matching)
            let at_word_boundary = if i > 0 {
                !chars[i-1].is_alphabetic()
            } else {
                true
            };

            // Only check for multi-letter patterns at word boundaries
            if at_word_boundary {
                // Check if this is a multi-letter known function first
                let letter_start = i;
                let mut temp_i = i;
                while temp_i < chars.len() && chars[temp_i].is_alphabetic() {
                    temp_i += 1;
                }
                let all_letters: String = chars[letter_start..temp_i].iter().collect();

                // Check if followed by parenthesis (function call)
                let has_paren = temp_i < chars.len() && chars[temp_i] == '(';

                // Check if it's a known function or defined multi-letter function
                if (KNOWN_FUNCTIONS.contains(&all_letters.as_str()) ||
                    defined_funcs.contains_key(&all_letters)) && has_paren {
                    // Process as multi-letter function
                    i = temp_i;

                    let mut function_arg = String::new();
                    function_arg.push(chars[i]);
                    i += 1;

                    let arg_start = i;
                    let mut paren_depth = 1;
                    while i < chars.len() && paren_depth > 0 {
                        if chars[i] == '(' {
                            paren_depth += 1;
                        } else if chars[i] == ')' {
                            paren_depth -= 1;
                            if paren_depth == 0 {
                                break;
                            }
                        }
                        i += 1;
                    }

                    let arg_content: String = chars[arg_start..i].iter().collect();
                    let processed_arg = process_definition_patterns(arg_content, eval_ctx, exclude_var);
                    function_arg.push_str(&processed_arg);

                    if i < chars.len() && chars[i] == ')' {
                        function_arg.push(')');
                        i += 1;
                    }

                    result.push_str(&format!("[{}{}]", all_letters, function_arg));
                    continue;
                }

                // Check if it's a multi-letter defined variable (no separator, no paren)
                let has_separator = temp_i < chars.len() && chars[temp_i] == VARIABLE_SEPARATOR;
                if !has_separator && all_letters.len() > 1 && defined_vars.contains_key(&all_letters) {
                    // Multi-letter defined variable
                    i = temp_i;
                    result.push_str(&format!("[{}]", all_letters));
                    continue;
                }
            }

            // Otherwise, process as single letter (valid variable naming scheme)
            let letter = chars[i];
            i += 1;

            // Check if single letter is a function call (letter followed by parenthesis)
            if i < chars.len() && chars[i] == '(' {
                let letter_str = letter.to_string();
                if defined_funcs.contains_key(&letter_str) {
                    // It's a function call
                    let mut function_arg = String::new();
                    function_arg.push(chars[i]);
                    i += 1;

                    let arg_start = i;
                    let mut paren_depth = 1;
                    while i < chars.len() && paren_depth > 0 {
                        if chars[i] == '(' {
                            paren_depth += 1;
                        } else if chars[i] == ')' {
                            paren_depth -= 1;
                            if paren_depth == 0 {
                                break;
                            }
                        }
                        i += 1;
                    }

                    let arg_content: String = chars[arg_start..i].iter().collect();
                    let processed_arg = process_definition_patterns(arg_content, eval_ctx, exclude_var);
                    function_arg.push_str(&processed_arg);

                    if i < chars.len() && chars[i] == ')' {
                        function_arg.push(')');
                        i += 1;
                    }

                    result.push_str(&format!("[{}{}]", letter, function_arg));
                    continue;
                }
            }

            // Check if followed by separator
            if i < chars.len() && chars[i] == VARIABLE_SEPARATOR {
                i += 1;
                let subscript_start = i;

                // Collect all alphanumeric characters after separator
                while i < chars.len() && chars[i].is_alphanumeric() {
                    i += 1;
                }

                let full_subscript: String = chars[subscript_start..i].iter().collect();

                if full_subscript.is_empty() {
                    // No subscript after separator - invalid, just output letter + separator
                    result.push(letter);
                    result.push(VARIABLE_SEPARATOR);
                    continue;
                }

                // Check for function call
                let mut function_arg = String::new();
                if i < chars.len() && chars[i] == '(' {
                    function_arg.push(chars[i]);
                    i += 1;

                    let arg_start = i;
                    let mut paren_depth = 1;
                    while i < chars.len() && paren_depth > 0 {
                        if chars[i] == '(' {
                            paren_depth += 1;
                        } else if chars[i] == ')' {
                            paren_depth -= 1;
                            if paren_depth == 0 {
                                break;
                            }
                        }
                        i += 1;
                    }

                    let arg_content: String = chars[arg_start..i].iter().collect();
                    let processed_arg = process_definition_patterns(arg_content, eval_ctx, exclude_var);
                    function_arg.push_str(&processed_arg);

                    if i < chars.len() && chars[i] == ')' {
                        function_arg.push(')');
                        i += 1;
                    }
                }

                // Try to greedily match the longest defined variable
                let mut matched = false;
                for len in (1..=full_subscript.len()).rev() {
                    let partial_subscript = &full_subscript[..len];
                    let candidate = format!("{}{}{}", letter, VARIABLE_SEPARATOR, partial_subscript);

                    // Check if it matches excluded variable
                    if let Some(excluded) = exclude_var {
                        if candidate == excluded {
                            result.push_str(&format!("[{}]{}", candidate, function_arg));
                            let remaining_subscript = &full_subscript[len..];
                            result.push_str(remaining_subscript);
                            matched = true;
                            break;
                        }
                    }

                    if defined_vars.contains_key(&candidate) {
                        result.push_str(&format!("[{}]", candidate));
                        let remaining_subscript = &full_subscript[len..];
                        result.push_str(remaining_subscript);
                        result.push_str(&function_arg);
                        matched = true;
                        break;
                    } else if defined_funcs.contains_key(&candidate) && !function_arg.is_empty() {
                        result.push_str(&format!("[{}{}]", candidate, function_arg));
                        let remaining_subscript = &full_subscript[len..];
                        result.push_str(remaining_subscript);
                        matched = true;
                        break;
                    }
                }

                if !matched {
                    // No match found, output as-is
                    // Check if single letter is defined
                    let letter_str = letter.to_string();
                    if defined_vars.contains_key(&letter_str) {
                        result.push_str(&format!("[{}]", letter));
                    } else {
                        result.push(letter);
                    }
                    result.push(VARIABLE_SEPARATOR);
                    result.push_str(&full_subscript);
                    result.push_str(&function_arg);
                }
            } else {
                // Just a single letter, no separator
                let letter_str = letter.to_string();

                // Check if it's excluded variable
                if let Some(excluded) = exclude_var {
                    if letter_str == excluded {
                        result.push_str(&format!("[{}]", letter));
                        continue;
                    }
                }

                if defined_vars.contains_key(&letter_str) {
                    result.push_str(&format!("[{}]", letter));
                } else {
                    result.push(letter);
                }
            }
        } else {
            // Not a lowercase letter at word boundary, just copy
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

