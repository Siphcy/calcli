#[allow(dead_code)]
use super::vi_inputs::History;
use crate::history_io::{export_history, import_history};
use color_eyre::Result;
use crate::eval::evaluate_input;
use crate::eval_context::EvalContext;
use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::crossterm::ExecutableCommand;
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Table, Row, Block, List, ListItem, Paragraph, ListState, TableState};
use ratatui::{DefaultTerminal, Frame};

//TODO: add window selection

pub struct InputHandler<'a> {
    input: String,
    character_index: usize,
    input_mode: InputMode,
    messages: Vec<String>,
    eval_ctx: EvalContext<'a>,
    history: History,
    messages_state: ListState,
    variables_state: TableState,
    last_key: Option<char>,
}

pub enum InputMode {
    Normal,
    Editing,
}

impl<'a> InputHandler<'a> {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            eval_ctx: EvalContext::new(),
            history: History::new(),
            character_index: 0,
            messages_state: ListState::default(),
            variables_state: TableState::default(),
            last_key: None,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn is_word_boundary(c: char) -> bool {
        c.is_whitespace() || "+-*/^%()[]{}.,=<>!&|".contains(c)
    }

    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn move_to_end_of_word(&mut self) {
        let chars: Vec<char> = self.input.chars().collect();
        if chars.is_empty() {
            return;
        }

        let mut pos = self.character_index;

        // If we're at the end already, do nothing
        if pos >= chars.len() - 1 {
            self.character_index = chars.len() - 1;
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

        self.character_index = pos.min(chars.len() - 1);
    }


    fn move_to_beginning_of_word(&mut self) {
        let chars: Vec<char> = self.input.chars().collect();
        if chars.is_empty() || self.character_index == 0 {
            return;
        }

        let mut pos = self.character_index;

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

        self.character_index = pos;
    }


    // History Handling
    fn add_to_history(&mut self) {
        self.history.add(&self.input);
    }

    fn get_previous_history(&mut self) {
        self.input.clear();
        if let Some(previous) = self.history.get_previous() {
            self.input = previous.to_string();
            self.character_index = previous.len();
        }
    }

    fn get_next_history(&mut self) {

        self.input.clear();
        if let Some(next) = self.history.get_next() {
            self.input = next.to_string();
            self.character_index = next.len();
        } else
        {
            self.character_index = 0;
        }
    }


   fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

   fn delete_char_indexed(&mut self) {

    let index = self.byte_index();
    if self.input.char_indices().map(|(i, _)| i).nth(index) != None {
        self.input.remove(index);

        }
            }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn scroll_messages_down(&mut self) {
        let i = match self.messages_state.selected() {
            Some(i) => {
                if i < self.messages.len().saturating_sub(1) {
                    i + 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.messages_state.select(Some(i));
    }
    fn scroll_messages_top(&mut self) {
        self.messages_state.select(Some(0))

    }
    fn scroll_messages_bottom(&mut self) {
        self.messages_state.select(Some(self.messages.len()));
    }

    fn scroll_messages_up(&mut self) {
        let i = match self.messages_state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    0
                }
            }
            None => 0,
        };
        self.messages_state.select(Some(i));
    }

    fn scroll_variables_down(&mut self) {
        let var_count = self.eval_ctx.defined_vars.iter()
            .filter(|(name, _)| !name.starts_with("lin"))
            .count();

        let i = match self.variables_state.selected() {
            Some(i) => {
                if i < var_count.saturating_sub(1) {
                    i + 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.variables_state.select(Some(i));
    }

    fn copy_selected_line(&mut self) {
        if let Some(index) = self.messages_state.selected() {
          if index < self.messages.len() {
              self.input = self.messages[index].clone()
                .split_once(") ")
                .and_then(|(_, rest)| rest.split_once(" = "))
                .map(|(expr, _)| expr)
                .unwrap_or("").to_string();

              self.character_index = self.input.len();
          }
      }
    }



    fn scroll_variables_up(&mut self) {
        let i = match self.variables_state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    0
                }
            }
            None => 0,
        };
        self.variables_state.select(Some(i));
    }

    fn submit_message(&mut self) {
        if self.input.is_empty() {
            self.input.clear();
            self.reset_cursor();
            return;
        }
        if self.input == "clear" {
            self.messages.clear();
            self.input.clear();
            self.reset_cursor();
            return;
        }

        // :w <filename> or :export <filename>
        if let Some(path) = self.input.strip_prefix(":w").or_else(|| self.input.strip_prefix(":export")) {
            let path = path.trim().to_string();
            if path.is_empty() {
                self.messages.push("Usage: :w <filename>  —  specify a file to export history to".to_string());
            } else {
                match export_history(&path, &self.eval_ctx.history_entries) {
                    Ok(()) => self.messages.push(format!("Exported history to {}", path)),
                    Err(e) => self.messages.push(format!("Export error: {}", e)),
                }
            }
            self.input.clear();
            self.reset_cursor();
            return;
        }

        // :r <filename> or :import <filename>
        if let Some(path) = self.input.strip_prefix(":r").or_else(|| self.input.strip_prefix(":import")) {
            let path = path.trim().to_string();
            if path.is_empty() {
                self.messages.push("Usage: :r <filename>  —  specify a file to import history from".to_string());
                self.input.clear();
                self.reset_cursor();
                return;
            }
            match import_history(&path) {
                Ok(entries) => {
                    for entry in entries {
                        match evaluate_input(&mut self.eval_ctx, &entry.expression) {
                            Ok(result) => {
                                self.messages.push(format!("{}) {} = {}", self.eval_ctx.counter, entry.expression.trim(), result));
                                self.eval_ctx.history_entries.push((entry.expression.clone(), result));
                            }
                            Err(e) => {
                                self.messages.push(format!("Import error on '{}': {}", entry.expression, e));
                            }
                        }
                    }
                    self.messages.push(format!("Imported history from {}", path));
                }
                Err(e) => self.messages.push(format!("Import error: {}", e)),
            }
            self.input.clear();
            self.reset_cursor();
            return;
        }

        self.add_to_history();
        match evaluate_input(&mut self.eval_ctx, &self.input.to_string()) {

            Ok(result) => {
                if self.input.starts_with("let ") {
                    let rest = self.input.strip_prefix("let ").unwrap();
                    let lhs = rest.split('=').next().unwrap_or("").trim();
                    if self.input.contains("[") && self.input.ends_with("]") {
                        let mut current_counter = self.eval_ctx.counter - self.eval_ctx.recently_assigned.iter().len();
                        for (def_name, def_value) in self.eval_ctx.recently_assigned.iter() {
                            current_counter += 1;
                            let msg = format!("{}) {} = {}", current_counter, def_name, def_value);

                                self.messages.push(format!("{}", msg,));
                        }

                    }
                    else if lhs.contains("(") && lhs.contains(")") {
                        if let Some((func_name, func)) = self.eval_ctx.defined_funcs.last() {
                            self.messages.push(format!("{}) {}({}) = {}", self.eval_ctx.counter, func_name, func.var_name, func.expr));
                        }
                    } else {
                        if let Some((var, value)) = self.eval_ctx.defined_vars.last() {
                            self.messages.push(format!("{}) {} = {}", self.eval_ctx.counter, var, value));
                        }
                    }
                }
                else {
                self.messages.push(format!("{}) {} = {}", self.eval_ctx.counter, self.input.trim(), result));
                }

            }
            Err(e) => {
                self.messages.push(format!("Error: {}", e));
            }
        }

        if !self.messages.is_empty() {
            self.messages_state.select(Some(self.messages.len().saturating_sub(1)));
        }

        let var_count = self.eval_ctx.defined_vars.iter()
            .filter(|(name, _)| !name.starts_with("lin"))
            .count();
        if var_count > 0 {
            self.variables_state.select(Some(var_count.saturating_sub(1)));
        }

        self.input.clear();
        self.reset_cursor();

    }



    pub fn preload_history(&mut self, path: &str) {
        match import_history(path) {
            Ok(entries) => {
                for entry in entries {
                    match evaluate_input(&mut self.eval_ctx, &entry.expression) {
                        Ok(result) => {
                            self.messages.push(format!("{}) {} = {}", self.eval_ctx.counter, entry.expression.trim(), result));
                            self.eval_ctx.history_entries.push((entry.expression, result));
                            self.eval_ctx.counter += 1;
                        }
                        Err(e) => {
                            self.messages.push(format!("Import error on '{}': {}", entry.expression, e));
                        }
                    }
                }
                self.messages.push(format!("Imported history from {}", path));
                if !self.messages.is_empty() {
                    self.messages_state.select(Some(self.messages.len().saturating_sub(1)));
                }
            }
            Err(e) => {
                self.messages.push(format!("Import error: {}", e));
            }
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            // Set cursor style based on mode
            let cursor_style = match self.input_mode {
                InputMode::Normal => SetCursorStyle::SteadyBlock,
                InputMode::Editing => SetCursorStyle::SteadyBar,
            };
            std::io::stdout().execute(cursor_style)?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            self.input_mode = InputMode::Editing;
                            self.last_key = None;
                        }
                        KeyCode::Char('a') => {
                            self.move_cursor_right();
                            self.input_mode = InputMode::Editing;
                            self.last_key = None;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        },
                        KeyCode::Char(':') => {
                            self.input.clear();
                            self.reset_cursor();
                            self.enter_char(':');
                            self.input_mode = InputMode::Editing;
                            self.last_key = None;
                        }
                        KeyCode::Char('h') | KeyCode::Left => {
                            self.move_cursor_left();
                            self.last_key = None;
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            self.move_cursor_right();
                            self.last_key = None;
                        }
                        KeyCode::Char('e') => {
                            self.move_to_end_of_word();
                            self.last_key = None;
                        }
                        KeyCode::Char('b') => {
                            self.move_to_beginning_of_word();
                            self.last_key = None;
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            self.get_previous_history();
                            self.last_key = None;
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            self.get_next_history();
                            self.last_key = None;
                        }
                        KeyCode::Esc => {
                            self.input.clear();
                            self.reset_cursor();
                            self.last_key = None;
                        },
                        KeyCode::Char('g') => {
                            if self.last_key == Some('g') {
                                self.scroll_messages_top();
                                self.last_key = None;
                            } else {
                                self.last_key = Some('g');
                            }
                        }
                        KeyCode::Char('G') => {
                            if self.last_key == Some('G') {
                                self.scroll_messages_bottom();
                                self.last_key = None;
                            } else {
                                self.last_key = Some('G');
                            }
                        }
                        // Scroll messages/results
                        KeyCode::Char('J') => {
                            self.scroll_messages_down();
                            self.last_key = None;
                        }
                        KeyCode::Char('K') => {
                            self.scroll_messages_up();
                            self.last_key = None;
                        }
                        // Scroll variables
                        KeyCode::Char('N') => {
                            self.scroll_variables_down();
                            self.last_key = None;
                        }
                        KeyCode::Char('P') => {
                            self.scroll_variables_up();
                            self.last_key = None;
                        }
                        KeyCode::Enter => {
                            self.submit_message();
                            self.last_key = None;
                        }
                        KeyCode::Char('y') => {
                            self.copy_selected_line();
                            self.last_key = None;
                        }
                        KeyCode::Char('x') => {
                            self.delete_char_indexed();
                            self.last_key = None;
                        }

                        _ => {
                            self.last_key = None;
                        }
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Up => {
                            self.get_previous_history();
                        }
                        KeyCode::Down => {
                            self.get_next_history();
                        }
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    }
    #[allow(deprecated)]
    fn render(&mut self, frame: &mut Frame) {
        let main_vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ]);
    let horizontal = Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)]);
    let definition_list_vertical = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]);

        let [help_area, messages_area, input_area, status_area] = main_vertical.areas(frame.area());
        let [output, def_area] = horizontal.areas(messages_area);
        let [var_list, func_list] = definition_list_vertical.areas(def_area);

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "i".bold(),
                    "/".into(),
                    "a".bold(),
                    " to edit, ".into(),
                    "k".bold(),
                    "/".into(),
                    "j".bold(),
                    " history, ".into(),
                    "J".bold(),
                    "/".into(),
                    "K".bold(),
                    " scroll results, ".into(),
                    "y".bold(),
                    " copy selected line, ".into(),
                    "gg".bold(),
                    "/".into(),
                    "gg".bold(),
                    " Jump through results, ".into(),
                    "N".bold(),
                    "/".into(),
                    "P".bold(),
                    " scroll vars.".into(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to evaluate, ".into(),
                    "↑/↓".bold(),
                    " history.".into(),


                ],
                Style::default(),
            ),
        };

        let status_msg = match self.input_mode {
            InputMode::Normal => "NORMAL",
            InputMode::Editing =>  "INSERT"
        };

        let text = Text::from(Line::from(msg)).patch_style(style);

        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);


        frame.render_widget(Block::bordered().title(status_msg), status_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Green),
            })
            .block(Block::bordered().title("Expression"));
        frame.render_widget(input, input_area);

        // Show cursor in both modes
        frame.set_cursor_position(Position::new(
            input_area.x + self.character_index as u16 + 1,
            input_area.y + 1,
        ));

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .map(|m| {
                let content = Line::from(Span::raw(m));
                ListItem::new(content)
            })
            .collect();

        let messages = List::new(messages)
            .block(Block::bordered().title("Results"))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol(">> ");

        let mut var_rows: Vec<Row> = Vec::new();
        for (name, value) in self.eval_ctx.defined_vars.iter() {
            if !name.starts_with("lin") {
                var_rows.push(Row::new(vec![name.clone(), value.to_string()]));
            }
        }
        let mut func_rows: Vec<Row> = Vec::new();
        for (name, func) in self.eval_ctx.defined_funcs.iter() {
                func_rows.push(Row::new(vec![format!("{}({})", name, func.var_name ), func.expr.clone()]));
        }

        let var_table = Table::new(
            var_rows,
            [Constraint::Percentage(20), Constraint::Percentage(80)]
        )
        .block(Block::bordered().title("Variables"))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">> ");
        let func_table = Table::new(
            func_rows,
            [Constraint::Percentage(20), Constraint::Percentage(80)]
        )
        .block(Block::bordered().title("Functions"))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">> ");

        frame.render_stateful_widget(func_table, var_list, &mut self.variables_state);
        frame.render_stateful_widget(var_table, func_list, &mut self.variables_state);
        frame.render_stateful_widget(messages, output, &mut self.messages_state);
    }
}
