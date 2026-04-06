pub mod eval;
pub mod eval_context;
pub mod tui_handler;
pub mod history_io;
pub mod definition_handler;
pub mod parser;
pub mod error;
pub mod conversion_handler;
pub mod constant;

// Variable name separator for numbered variables
// e.g., with VARIABLE_SEPARATOR = '_':
//   x_1, x_2, f_10, lin_5
// Change this to use a different separator (e.g., '-' for x-1, x-2)
pub const VARIABLE_SEPARATOR: char = '_';

// Helper function to escape the separator for regex patterns
pub fn escape_separator() -> String {
    // Escape regex metacharacters: . ^ $ * + ? ( ) [ ] { } | \
    match VARIABLE_SEPARATOR {
        '.' | '^' | '$' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\' => {
            format!("\\{}", VARIABLE_SEPARATOR)
        }
        _ => VARIABLE_SEPARATOR.to_string()
    }
}
