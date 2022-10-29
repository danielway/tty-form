use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{pos, Interface, Position};
use tty_text::Key;

use crate::{
    dependency::DependencyState,
    style::{error_style, help_style},
    text::{set_segment_subset_style, DrawerContents, Segment, Text},
    utility::render_segment,
    Form,
};

use super::{InputResult, Step};

/// A multi-line text input step.
///
/// # Examples
/// ```
/// use tty_form::{
///     Form,
///     step::{Step, TextBlockStep},
/// };
///
/// let mut form = Form::new();
///
/// let mut step = TextBlockStep::new("Enter your story:");
/// step.set_max_line_length(100);
/// step.add_to(&mut form);
/// ```
pub struct TextBlockStep {
    prompt: String,
    text: tty_text::Text,
    top_margin: Option<u16>,
    bottom_margin: Option<u16>,
    max_line_length: Option<u16>,
    trim_trailing_whitespace: bool,
}

impl TextBlockStep {
    /// Create a new, default text block step.
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            text: tty_text::Text::new(true),
            top_margin: None,
            bottom_margin: None,
            max_line_length: None,
            trim_trailing_whitespace: true,
        }
    }

    /// Set this text block's top and bottom margins.
    pub fn set_margins(&mut self, top_margin: Option<u16>, bottom_margin: Option<u16>) {
        self.top_margin = top_margin;
        self.bottom_margin = bottom_margin;
    }

    /// Set this text block step's optional maximum line grapheme length.
    pub fn set_max_line_length(&mut self, max_length: u16) {
        self.max_line_length = Some(max_length);
    }

    /// Set whether this text block should trim trailing whitespace.
    pub fn set_trim_trailing_whitespace(&mut self, trim: bool) {
        self.trim_trailing_whitespace = trim;
    }
}

impl Step for TextBlockStep {
    fn initialize(&mut self, _dependency_state: &mut DependencyState, _index: usize) {}

    fn render(
        &self,
        interface: &mut Interface,
        _dependency_state: &DependencyState,
        position: Position,
        is_focused: bool,
    ) -> u16 {
        if !is_focused && self.text.value().is_empty() {
            return 1;
        }

        let mut offset_y = 0;
        if let Some(top_margin) = self.top_margin {
            for line in 0..top_margin {
                interface.clear_line(position.y() + line);
            }

            offset_y += top_margin;
        }

        let lines = self.text.lines();
        for (line_index, line) in lines.iter().enumerate() {
            let line_position = pos!(0, position.y() + line_index as u16 + offset_y);

            // If the line exceeds the max length, render the tail as an error
            if let Some(max_length) = self.max_line_length {
                let line_length = line.len() as u16;
                if line_length > max_length {
                    let mut segment = Text::new(line.to_string()).as_segment();

                    set_segment_subset_style(
                        &mut segment,
                        max_length.into(),
                        line_length.into(),
                        error_style(),
                    );

                    render_segment(interface, line_position, segment);
                    continue;
                }
            }

            interface.set(line_position, line);
        }

        if is_focused {
            let cursor = self.text.cursor();
            let (x, y) = (cursor.0 as u16, cursor.1 as u16);
            interface.set_cursor(Some(pos!(x, y + position.y() + offset_y)));
        }

        if let Some(bottom_margin) = self.bottom_margin {
            for line in 0..bottom_margin {
                interface.clear_line(position.y() + line + offset_y + lines.len() as u16);
            }

            offset_y += bottom_margin;
        }

        lines.len() as u16 + offset_y
    }

    fn update(
        &mut self,
        _dependency_state: &mut DependencyState,
        input: KeyEvent,
    ) -> Option<InputResult> {
        // If there are two empty lines, advance the form
        if input.code == KeyCode::Enter || input.code == KeyCode::Tab {
            let lines = self.text.lines().to_vec();
            if lines.len() >= 2 {
                let last_lines_empty =
                    lines[lines.len() - 1].is_empty() && lines[lines.len() - 2].is_empty();

                if last_lines_empty {
                    // If we're trailing whitespace, delete the last two blank lines
                    if self.trim_trailing_whitespace {
                        self.text.handle_input(Key::Backspace);
                        self.text.handle_input(Key::Backspace);
                    }

                    return Some(InputResult::AdvanceForm);
                }
            }
        }

        if input.code == KeyCode::Esc || input.code == KeyCode::BackTab {
            return Some(InputResult::RetreatForm);
        }

        match input.code {
            KeyCode::Enter => self.text.handle_input(Key::Enter),
            KeyCode::Char(ch) => self.text.handle_input(Key::Char(ch)),
            KeyCode::Backspace => self.text.handle_input(Key::Backspace),
            KeyCode::Up => self.text.handle_input(Key::Up),
            KeyCode::Down => self.text.handle_input(Key::Down),
            KeyCode::Left => self.text.handle_input(Key::Left),
            KeyCode::Right => self.text.handle_input(Key::Right),
            _ => {}
        };

        None
    }

    fn help(&self) -> Segment {
        Text::new_styled(self.prompt.to_string(), help_style()).as_segment()
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn result(&self, _dependency_state: &DependencyState) -> String {
        if self.text.value().is_empty() {
            return "\n".to_string();
        }

        let mut result = String::new();

        if let Some(top_margin) = self.top_margin {
            for _ in 0..top_margin {
                result.push('\n');
            }
        }

        result.push_str(&self.text.value());

        if let Some(bottom_margin) = self.bottom_margin {
            for _ in 0..bottom_margin + 1 {
                result.push('\n');
            }
        }

        result
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}
