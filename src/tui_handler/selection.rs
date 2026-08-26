use ratatui::style::Color;

pub enum LastMove {
    UP,
    DOWN,
    RIGHT,
    LEFT,
    NONE,
}

pub struct TuiSelectionMap {
     width: usize,
     height: usize,
     pub curr_pos: (usize, usize),
     pub layout: Vec<Vec<&'static str>>,
     last_win: String,
     last_move: LastMove
 }

impl TuiSelectionMap {
    pub fn new() -> Self {
        Self {
            width: 2,
            height: 3,
            curr_pos: (0, 0),
            layout: vec![
            //Maps TUI to virtual 2d Vector
            //Map is flipped such that bottom left is top left
            //IF YOU WANT TO MERGE 2 boxes vertically or horizontally use MERGE_UP, MERGE_DOWN, MERGE_LEFT, and MERGE_RIGHT

                vec![ "INPUT_WIN", "INPUT_WIN"],

                vec![ "RESULT_WIN", "VARIABLE_WIN"],

                vec![ "RESULT_WIN", "FUNCTION_WIN"],
            ],
            last_win: String::new(),
            last_move: LastMove::NONE,
        }


    }
    pub fn move_up(&mut self) {
        let start_win = self.layout[self.curr_pos.1][self.curr_pos.0];

        // Try to move up
        if self.curr_pos.1 < self.height - 1 {
            self.curr_pos.1 += 1;

            // Skip over any cells that are the same window (merged cells)
            while self.curr_pos.1 < self.height - 1
                && self.layout[self.curr_pos.1][self.curr_pos.0] == start_win {
                self.curr_pos.1 += 1;
            }

            // If we're still in the same window after moving, we hit the boundary
            if self.layout[self.curr_pos.1][self.curr_pos.0] == start_win {
                // Out of bounds - wrap to right column at bottom
                if self.curr_pos.0 < self.width - 1 {
                    self.curr_pos.0 += 1;
                    self.curr_pos.1 = 0;
                    self.last_move = LastMove::RIGHT;
                } else {
                    // Restore position and stop
                    self.curr_pos.1 -= 1;
                    self.last_move = LastMove::NONE;
                }
            } else {
                self.last_move = LastMove::UP;
            }
        } else {
            // Out of bounds - wrap to right column at bottom
            if self.curr_pos.0 < self.width - 1 {
                self.curr_pos.0 += 1;
                self.curr_pos.1 = 0;
                self.last_move = LastMove::RIGHT;
            } else {
                self.last_move = LastMove::NONE;
            }
        }
    }

    pub fn move_down(&mut self) {
        let start_win = self.layout[self.curr_pos.1][self.curr_pos.0];

        // Try to move down
        if self.curr_pos.1 > 0 {
            self.curr_pos.1 -= 1;

            // Skip over any cells that are the same window (merged cells)
            while self.curr_pos.1 > 0
                && self.layout[self.curr_pos.1][self.curr_pos.0] == start_win {
                self.curr_pos.1 -= 1;
            }

            // If we're still in the same window after moving, we hit the boundary
            if self.layout[self.curr_pos.1][self.curr_pos.0] == start_win {
                // Out of bounds - wrap to left column at top
                if self.curr_pos.0 > 0 {
                    self.curr_pos.0 -= 1;
                    self.curr_pos.1 = self.height - 1;
                    self.last_move = LastMove::LEFT;
                } else {
                    // Restore position and stop
                    self.curr_pos.1 += 1;
                    self.last_move = LastMove::NONE;
                }
            } else {
                self.last_move = LastMove::DOWN;
            }
        } else {
            // Out of bounds - wrap to left column at top
            if self.curr_pos.0 > 0 {
                self.curr_pos.0 -= 1;
                self.curr_pos.1 = self.height - 1;
                self.last_move = LastMove::LEFT;
            } else {
                self.last_move = LastMove::NONE;
            }
        }
    }

    pub fn move_left(&mut self) {
        let start_win = self.layout[self.curr_pos.1][self.curr_pos.0];

        // Try to move left
        if self.curr_pos.0 > 0 {
            self.curr_pos.0 -= 1;

            // Skip over any cells that are the same window (merged cells)
            while self.curr_pos.0 > 0
                && self.layout[self.curr_pos.1][self.curr_pos.0] == start_win {
                self.curr_pos.0 -= 1;
            }

            // If we're still in the same window after moving, we hit the boundary
            if self.layout[self.curr_pos.1][self.curr_pos.0] == start_win {
                // Out of bounds left - move down
                if self.curr_pos.1 > 0 {
                    self.curr_pos.1 -= 1;
                    self.curr_pos.0 = self.width - 1;
                    self.last_move = LastMove::DOWN;
                } else {
                    // Restore position and stop
                    self.curr_pos.0 += 1;
                    self.last_move = LastMove::NONE;
                }
            } else {
                self.last_move = LastMove::LEFT;
            }
        } else {
            // Out of bounds left - move down
            if self.curr_pos.1 > 0 {
                self.curr_pos.1 -= 1;
                self.curr_pos.0 = self.width - 1;
                self.last_move = LastMove::DOWN;
            } else {
                self.last_move = LastMove::NONE;
            }
        }
    }

    pub fn move_right(&mut self) {
        let start_win = self.layout[self.curr_pos.1][self.curr_pos.0];

        // Try to move right
        if self.curr_pos.0 < self.width - 1 {
            self.curr_pos.0 += 1;

            // Skip over any cells that are the same window (merged cells)
            while self.curr_pos.0 < self.width - 1
                && self.layout[self.curr_pos.1][self.curr_pos.0] == start_win {
                self.curr_pos.0 += 1;
            }

            // If we're still in the same window after moving, we hit the boundary
            if self.layout[self.curr_pos.1][self.curr_pos.0] == start_win {
                // Out of bounds right - move up
                if self.curr_pos.1 < self.height - 1 {
                    self.curr_pos.1 += 1;
                    self.curr_pos.0 = 0;
                    self.last_move = LastMove::UP;
                } else {
                    // Restore position and stop
                    self.curr_pos.0 -= 1;
                    self.last_move = LastMove::NONE;
                }
            } else {
                self.last_move = LastMove::RIGHT;
            }
        } else {
            // Out of bounds right - move up
            if self.curr_pos.1 < self.height - 1 {
                self.curr_pos.1 += 1;
                self.curr_pos.0 = 0;
                self.last_move = LastMove::UP;
            } else {
                self.last_move = LastMove::NONE;
            }
        }
    }
    pub fn get_pos(&self) -> (usize, usize) {
        return self.curr_pos
    }

    pub fn update_selection(&mut self) -> &'static str {
        let current_win = self.layout[self.curr_pos.1][self.curr_pos.0];
        self.last_win = current_win.to_string();
        current_win
    }

    pub fn selected_color(&mut self, window: &'static str) -> Color {
        if self.update_selection() == window {
            return Color::LightBlue;
        }
        return Color::White

    }

    pub fn selected(&mut self, window: &'static str) -> bool {
        if self.update_selection() == window {
            return true;
        }
        return false

    }

    pub fn reset_selection(&mut self) {
        self.curr_pos = (0, 0)
    }



}
