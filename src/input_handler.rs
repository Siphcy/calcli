//TODO: handles inputs and proactively checks for \ for latex commands
//and additioanlly autoamtically prompts:
//- variable format (helper function)
//- function evaluation (helper function)
//- symbol change (native)
//- latex -> unicode (native)
//- implicit_multiplication (helper function)
//

/// Maps LaTeX commands to Unicode symbols
pub fn latex_to_unicode_symbol(expression: &str) -> Option<&'static str> {
    match expression {
        // Greek letters (lowercase)
        "pi" => Some("π"),
        "alpha" => Some("α"),
        "beta" => Some("β"),
        "gamma" => Some("γ"),
        "delta" => Some("δ"),
        "epsilon" => Some("ε"),
        "zeta" => Some("ζ"),
        "eta" => Some("η"),
        "theta" => Some("θ"),
        "iota" => Some("ι"),
        "kappa" => Some("κ"),
        "lambda" => Some("λ"),
        "mu" => Some("μ"),
        "nu" => Some("ν"),
        "xi" => Some("ξ"),
        "omicron" => Some("ο"),
        "rho" => Some("ρ"),
        "sigma" => Some("σ"),
        "tau" => Some("τ"),
        "upsilon" => Some("υ"),
        "phi" => Some("φ"),
        "chi" => Some("χ"),
        "psi" => Some("ψ"),
        "omega" => Some("ω"),

        // Greek letters (uppercase)
        "Gamma" => Some("Γ"),
        "Delta" => Some("Δ"),
        "Theta" => Some("Θ"),
        "Lambda" => Some("Λ"),
        "Xi" => Some("Ξ"),
        "Pi" => Some("Π"),
        "Sigma" => Some("Σ"),
        "Phi" => Some("Φ"),
        "Psi" => Some("Ψ"),
        "Omega" => Some("Ω"),



        _ => None,
    }
}

pub struct InputFormat {
    cursor_index: usize,
    input: String,
    latex_str: String,
    latex_arg: bool,
}

impl InputFormat {
    pub fn new() -> Self {
        Self {
            cursor_index: 0,
            input: String::new(),
            latex_str: String::new(),
            latex_arg: false,
        }
    }

    // Getters
    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn cursor_index(&self) -> usize {
        self.cursor_index
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
        self.cursor_index = self.input.chars().count();
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor_index = 0;
        self.latex_str.clear();
        self.latex_arg = false;
    }

    // Cursor movement
    fn clamp_cursor(&self, new_pos: usize) -> usize {
        new_pos.clamp(0, self.input.chars().count())
    }

    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(self.input.len())
    }


    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_right);
    }

    fn is_word_boundary(c: char) -> bool {
        c.is_whitespace() || "+-*/^%()[]{}.,=<>!&|".contains(c)
    }

    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    pub fn move_to_end_of_word(&mut self) {
        let chars: Vec<char> = self.input.chars().collect();
        if chars.is_empty() {
            return;
        }

        let mut pos = self.cursor_index;

        // If we're at the end already, do nothing
        if pos >= chars.len() - 1 {
            self.cursor_index = chars.len() - 1;
            return;
        }

        // Skip current character to start moving
        pos += 1;

        // Skip any boundaries/whitespace
        while pos < chars.len() && Self::is_word_boundary(chars[pos]) {
            pos += 1;
        }

        // Move to end of the word
        while pos < chars.len() && Self::is_word_char(chars[pos]) {
            pos += 1;
        }

        // Back up one to land on last character of word
        if pos > 0 {
            pos -= 1;
        }

        self.cursor_index = pos.min(chars.len() - 1);
    }

    pub fn move_to_beginning_of_word(&mut self) {
        let chars: Vec<char> = self.input.chars().collect();
        if chars.is_empty() || self.cursor_index == 0 {
            return;
        }

        let mut pos = self.cursor_index;

        // Move back one to start
        pos = pos.saturating_sub(1);

        // Skip boundaries/whitespace backwards
        while pos > 0 && Self::is_word_boundary(chars[pos]) {
            pos -= 1;
        }

        // Move to start of word
        while pos > 0 && Self::is_word_char(chars[pos - 1]) {
            pos -= 1;
        }

        self.cursor_index = pos;
    }

    // Character manipulation
    pub fn enter_char(&mut self, input_char: char) {
        // Handle LaTeX conversion
        if input_char == '\\' {
            self.latex_arg = true;
        }

        if input_char.is_whitespace() && self.latex_arg {
            // Remove the backslash and latex command
            let chars_to_remove = self.latex_str.len();
                        // Insert the unicode symbol if found
            if let Some(symbol) = latex_to_unicode_symbol(&self.latex_str.trim_start_matches('\\')) {

            for _ in 0..chars_to_remove {
                if !self.input.is_empty() {
                    self.input.remove(self.input.len() - 1);
                    self.cursor_index = self.cursor_index.saturating_sub(1);
                }
            }


                let index = self.byte_index();
                self.input.insert_str(index, symbol);
                self.cursor_index += symbol.chars().count();
            }

            self.latex_arg = false;
            self.latex_str.clear();
            return;
        }

        // Track latex string
        if self.latex_arg {
            self.latex_str.push(input_char);
        }

        // Insert character at cursor position
        let index = self.byte_index();
        self.input.insert(index, input_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();

            // Reset latex tracking if we're deleting
            if self.latex_arg && !self.latex_str.is_empty() {
                self.latex_str.pop();
            }
        }
    }

    pub fn delete_char_indexed(&mut self) {
        let index = self.byte_index();
        if self.input.char_indices().nth(index).is_some() {
            self.input.remove(index);
        }
    }
}







