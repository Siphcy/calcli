

pub struct History {
    history_vec: Vec<String>,
    current_index: Option<usize>, // None means we're not navigating history
}

impl History {
    pub fn new() -> Self {
        Self {
            history_vec: Vec::new(),
            current_index: None,
        }
    }

    pub fn add(&mut self, expr: &String) {
        self.history_vec.push(expr.clone());
        self.current_index = None; // Reset navigation when new entry added
    }

    pub fn get_previous(&mut self) -> Option<&String> {
        if self.history_vec.is_empty() {
            return None;
        }

        let new_index = match self.current_index {
            None => self.history_vec.len() - 1, // Start from most recent
            Some(0) => 0, // Already at oldest, stay there
            Some(i) => i - 1, // Go back one
        };

        self.current_index = Some(new_index);
        self.history_vec.get(new_index)
    }

    // Navigate forwards in history (down arrow)
    pub fn get_next(&mut self) -> Option<&String> {
        match self.current_index {
            None => None, // Not navigating history
            Some(i) if i >= self.history_vec.len() - 1 => {
                // At newest entry, go back to "present" (None)
                self.current_index = None;
                None
            }
            Some(i) => {
                let new_index = i + 1;
                self.current_index = Some(new_index);
                self.history_vec.get(new_index)
            }
        }
    }

    // Get the last (most recent) entry without changing navigation state
    pub fn get_last(&self) -> Option<&String> {
        self.history_vec.last()
    }

    // Get current history position
    pub fn current(&self) -> Option<&String> {
        self.current_index.and_then(|i| self.history_vec.get(i))
    }

    // Reset navigation to "present"
    pub fn reset_navigation(&mut self) {
        self.current_index = None;
    }
}
