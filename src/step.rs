use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{pos, Interface, Position};
use tty_text::{Key, Text};

use crate::{Control, Form};

/// A distinct, vertically-separated phase of the form.
pub trait Step {
    /// Perform any post-configuration initialization actions for this step.
    fn initialize(&mut self);

    /// Render this step at the specified position and return the height of the rendered content.
    fn render(&mut self, position: Position, interface: &mut Interface, is_focused: bool) -> u16;

    /// Handle the specified input event, optionally returning an instruction for the form.
    fn handle_input(&mut self, event: KeyEvent) -> Option<InputResult>;

    /// Retrieve this step's current help text.
    fn get_help_text(&self) -> String;

    /// Retrieve this step's current drawer contents, if applicable.
    fn get_drawer(&self) -> Option<Vec<String>>;

    /// Complete configuration and add this step to the form.
    fn add_to_form(self, form: &mut Form);

    /// Retrieves this step's final WYSIWYG result.
    fn get_result(&self) -> String;
}

/// After processing an input event, an action may be returned to the form from the step.
pub enum InputResult {
    /// Advance the form to the next step.
    AdvanceForm,
    /// Retreat the form to the previous step.
    RetreatForm,
}

/// A single-line step which controls multple controls including static and input elements.
///
/// # Examples
/// ```
/// use tty_form::{Form, Step, CompoundStep, Control, StaticText, TextInput};
///
/// let mut form = Form::new();
///
/// let mut step = CompoundStep::new();
/// StaticText::new("A Form - ").add_to_step(&mut step);
/// TextInput::new("Enter your name:", false).add_to_step(&mut step);
/// step.add_to_form(&mut form);
/// ```
pub struct CompoundStep {
    controls: Vec<Box<dyn Control>>,
    max_line_length: Option<u8>,
    active_control: usize,
}

impl CompoundStep {
    /// Create a new compound step with no controls.
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            max_line_length: None,

            active_control: 0,
        }
    }

    /// Append the specified control to this step.
    pub fn add_control(&mut self, control: Box<dyn Control>) {
        self.controls.push(control);
    }

    /// Set this step's maximum total line length.
    pub fn set_max_line_length(&mut self, max_length: u8) {
        self.max_line_length = Some(max_length);
    }

    /// Advance the step's state to the next control. Returns true if we've reached the end of this
    /// step and the form should advance to the next.
    fn advance_control(&mut self) -> bool {
        loop {
            if self.active_control + 1 >= self.controls.len() {
                return true;
            }

            self.active_control += 1;

            if self.controls[self.active_control].is_focusable() {
                break;
            }
        }

        false
    }

    /// Retreat the step's state to the previous control. Returns true if we've reached the start
    /// of this step and the form should retreat to the previous.
    fn retreat_control(&mut self) -> bool {
        loop {
            if self.active_control == 0 {
                return true;
            }

            self.active_control -= 1;

            if self.controls[self.active_control].is_focusable() {
                break;
            }
        }

        false
    }
}

impl Step for CompoundStep {
    fn initialize(&mut self) {
        if !self.controls[0].is_focusable() {
            self.advance_control();
        }
    }

    fn render(&mut self, mut position: Position, interface: &mut Interface, is_focused: bool) -> u16 {
        interface.clear_line(position.y());

        let mut cursor_position = None;
        for (control_index, control) in self.controls.iter().enumerate() {
            let (text, cursor_offset) = control.get_text();
            interface.set(position, &text);

            if control_index == self.active_control {
                if let Some(offset) = cursor_offset {
                    cursor_position = Some(pos!(position.x() + offset, position.y()));
                }
            }

            position = pos!(position.x() + text.len() as u16, position.y());
        }

        if is_focused {
            interface.set_cursor(cursor_position);
        }

        1
    }

    fn handle_input(&mut self, key_event: KeyEvent) -> Option<InputResult> {
        match (key_event.modifiers, key_event.code) {
            (_, KeyCode::Enter) => {
                if self.advance_control() {
                    return Some(InputResult::AdvanceForm);
                }
            }
            (_, KeyCode::Esc) => {
                if self.retreat_control() {
                    return Some(InputResult::RetreatForm);
                }
            }
            _ => self.controls[self.active_control].handle_input(key_event),
        }

        None
    }

    fn get_help_text(&self) -> String {
        self.controls[self.active_control].get_help().unwrap_or(String::new())
    }

    fn get_drawer(&self) -> Option<Vec<String>> {
        self.controls[self.active_control].get_drawer()
    }

    fn add_to_form(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }

    fn get_result(&self) -> String {
        let mut result = String::new();

        for control in &self.controls {
            let (text, _) = control.get_text();
            result.push_str(&text);
        }

        result.push('\n');

        result
    }
}

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
/// step.add_to_form(&mut form);
/// ```
pub struct TextBlockStep {
    prompt: String,
    text: Text,
    max_line_length: Option<u8>,
}

impl TextBlockStep {
    /// Create a new, default text block step.
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            text: Text::new(true),
            max_line_length: None,
        }
    }

    /// Set this text block step's optional maximum line grapheme length.
    pub fn set_max_line_length(&mut self, max_length: u8) {
        self.max_line_length = Some(max_length);
    }
}

impl Step for TextBlockStep {
    fn initialize(&mut self) {}

    fn render(&mut self, position: Position, interface: &mut Interface, is_focused: bool) -> u16 {
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

    fn handle_input(&mut self, event: KeyEvent) -> Option<InputResult> {
        if event.code == KeyCode::Enter {
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

        if event.code == KeyCode::Esc {
            return Some(InputResult::RetreatForm);
        }

        match event.code {
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

    fn get_help_text(&self) -> String {
        self.prompt.to_string()
    }

    fn get_drawer(&self) -> Option<Vec<String>> {
        None
    }

    fn add_to_form(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }

    fn get_result(&self) -> String {
        let mut result = self.text.value();
        result.push('\n');
        result
    }
}
