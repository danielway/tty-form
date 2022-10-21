use crossterm::event::{KeyEvent, KeyCode};
use tty_interface::{Interface, Position};

use crate::{Control, SelectInput, StaticText, TextInput};

/// A distinct, vertically-separated phase of the form.
pub trait Step {
    /// Render this step at the specified position.
    fn render(&mut self, position: Position, interface: &mut Interface);
    
    /// Handle the specified input event, optionally returning an instruction for the form.
    fn handle_input(&mut self, event: KeyEvent) -> Option<InputResult>;
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
    controls: Vec<Control>,
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
    pub fn add_control(&mut self, control: Control) -> &mut Control {
        self.controls.push(control);
        self.controls.last_mut().unwrap()
    }

    /// Append and return a new static text control.
    pub fn add_static_text(&mut self) -> &mut StaticText {
        match self.add_control(Control::Static(StaticText::new())) {
            Control::Static(control) => control,
            _ => panic!(),
        }
    }

    /// Append and return a new text input control.
    pub fn add_text_input(&mut self) -> &mut TextInput {
        match self.add_control(Control::Text(TextInput::new())) {
            Control::Text(control) => control,
            _ => panic!(),
        }
    }

    /// Append and return a new option selection input control.
    pub fn add_select_input(&mut self) -> &mut SelectInput {
        match self.add_control(Control::Select(SelectInput::new())) {
            Control::Select(control) => control,
            _ => panic!(),
        }
    }

    /// Set this step's maximum total line length.
    pub fn set_max_line_length(&mut self, max_length: u8) {
        self.max_line_length = Some(max_length);
    }
}

impl Step for CompoundStep {
    fn render(&mut self, position: Position, interface: &mut Interface) {
        interface.set(position, &format!("CompoundStep: on control {} of {}", self.active_control + 1, self.controls.len()));
    }

    fn handle_input(&mut self, event: KeyEvent) -> Option<InputResult> {
        match (event.modifiers, event.code) {
            (_, KeyCode::Enter) => {
                if self.active_control + 1 == self.controls.len() {
                    return Some(InputResult::AdvanceForm);
                } else {
                    self.active_control += 1;
                }
            }
            (_, KeyCode::Esc) => {
                if self.active_control == 0 {
                    return Some(InputResult::RetreatForm);
                } else {
                    self.active_control -= 1;
                }
            }
            // TODO: forwarding input to individual controls
            _ => {},
        }

        None
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
    max_line_length: Option<u8>,
}

impl TextBlockStep {
    /// Create a new, default text block step.
    pub fn new() -> Self {
        Self {
            max_line_length: None,
        }
    }

    /// Set this text block step's optional maximum line grapheme length.
    pub fn set_max_line_length(&mut self, max_length: u8) {
        self.max_line_length = Some(max_length);
    }
}

impl Step for TextBlockStep {
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
}