use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{pos, Interface, Position};
use tty_text::Key;

use crate::{
    dependency::DependencyState,
    text::{DrawerContents, Segment, Text},
    Form,
};

use super::{InputResult, Step};

/// A multi-line text input step.
///
/// # Examples
/// ```
/// use tty_form::{Form, Step, TextBlockStep};
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
    max_line_length: Option<u8>,
}

impl TextBlockStep {
    /// Create a new, default text block step.
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            text: tty_text::Text::new(true),
            max_line_length: None,
        }
    }

    /// Set this text block step's optional maximum line grapheme length.
    pub fn set_max_line_length(&mut self, max_length: u8) {
        self.max_line_length = Some(max_length);
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
        let lines = self.text.lines();
        for (line_index, line) in lines.iter().enumerate() {
            interface.set(pos!(0, position.y() + line_index as u16), line);
        }

        if is_focused {
            let cursor = self.text.cursor();
            let (x, y) = (cursor.0 as u16, cursor.1 as u16);
            interface.set_cursor(Some(pos!(x, y + position.y())));
        }

        lines.len() as u16
    }

    fn update(
        &mut self,
        _dependency_state: &mut DependencyState,
        input: KeyEvent,
    ) -> Option<InputResult> {
        if input.code == KeyCode::Enter {
            let mut last_two_empty = self.text.lines().iter().count() > 2;
            if last_two_empty {
                for line in self.text.lines().iter().rev().take(2) {
                    if !line.is_empty() {
                        last_two_empty = false;
                    }
                }
            }

            if last_two_empty {
                return Some(InputResult::AdvanceForm);
            }
        }

        if input.code == KeyCode::Esc {
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
        Text::new(self.prompt.to_string()).as_segment()
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn result(&self, _dependency_state: &DependencyState) -> String {
        let mut result = self.text.value();
        result.push('\n');
        result
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}
