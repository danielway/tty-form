use crossterm::event::{KeyEvent, KeyCode};
use tty_interface::{Interface, Position, pos};

use crate::{Control, Form};

/// A distinct, vertically-separated phase of the form.
pub trait Step {
    fn initialize(&mut self);

    /// Render this step at the specified position.
    fn render(&mut self, position: Position, interface: &mut Interface);
    
    /// Handle the specified input event, optionally returning an instruction for the form.
    fn handle_input(&mut self, event: KeyEvent) -> Option<InputResult>;

    fn add_to_form(self, form: &mut Form);
}

pub enum InputResult {
    AdvanceForm,
    RetreatForm,
}

/// A single-line step which controls multple controls including static and input elements.
///
/// # Examples
/// ```
/// use tty_form::Form;
///
/// let mut form = Form::default();
/// let mut step = form.add_compound_step();
///
/// let text = step.add_static_text();
/// text.set_text("Branch: ");
///
/// let input = step.add_text_input();
/// input.set_force_lowercase(true);
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

    fn advance_control(&mut self) -> bool {
        loop {
            if self.active_control + 1 >= self.controls.len() {
                return true;
            }
            
            self.active_control += 1;

            if self.controls[self.active_control].is_focusable() {
                break
            }
        }

        false
    }

    fn retreat_control(&mut self) -> bool {
        loop {
            if self.active_control == 0 {
                return true;
            }
            
            self.active_control -= 1;

            if self.controls[self.active_control].is_focusable() {
                break
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
    
    fn render(&mut self, mut position: Position, interface: &mut Interface) {
        let mut cursor_position = None;

        for (control_index, control) in self.controls.iter().enumerate() {
            let (text, cursor_offset) = control.get_text();
            
            if control_index == self.active_control {
                if let Some(offset) = cursor_offset {
                    cursor_position = Some(pos!(position.x() + offset, position.y()));
                }
            }

            interface.set(position, &text);
            
            position = pos!(position.x() + text.len() as u16, position.y());
        }

        interface.set_cursor(cursor_position);
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

    fn add_to_form(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}

/// A multi-line text input step.
///
/// # Examples
/// ```
/// use tty_form::Form;
///
/// let mut form = Form::default();
/// let mut step = form.add_text_block_step();
/// step.set_max_line_length(100);
/// ```
pub struct TextBlockStep {
    prompt: String,
    max_line_length: Option<u8>,
}

impl TextBlockStep {
    /// Create a new, default text block step.
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
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

    fn render(&mut self, position: Position, interface: &mut Interface) {
        interface.set(position, "TextBlockStep");
    }

    fn handle_input(&mut self, event: KeyEvent) -> Option<InputResult> {
        match (event.modifiers, event.code) {
            (_, KeyCode::Enter) => {
                // TODO: advance iff there's already a blank line above the cursor
                Some(InputResult::AdvanceForm)
            }
            (_, KeyCode::Esc) => {
                Some(InputResult::RetreatForm)
            }
            // TODO: forwarding input to individual controls
            _ => None,
        }
    }

    fn add_to_form(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}