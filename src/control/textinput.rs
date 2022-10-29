use crossterm::event::{KeyCode, KeyEvent};
use tty_text::Key;

use crate::{
    dependency::{Action, DependencyId, Evaluation},
    step::CompoundStep,
    style::help_style,
    text::{DrawerContents, Segment, Text},
};

use super::Control;

/// A single-line text field input. May be used as an evaluation for dependent form elements.
///
/// # Examples
/// ```
/// use tty_form::{
///     step::CompoundStep,
///     control::{Control, TextInput},
/// };
///
/// let mut step = CompoundStep::new();
/// TextInput::new("Enter your name:", false).add_to(&mut step);
/// ```
pub struct TextInput {
    prompt: String,
    text: tty_text::Text,
    force_lowercase: bool,
    evaluation: Option<(DependencyId, Evaluation)>,
}

impl TextInput {
    /// Create a new text input control with the specified prompt and casing-rules.
    pub fn new(prompt: &str, force_lowercase: bool) -> Self {
        Self {
            prompt: prompt.to_string(),
            text: tty_text::Text::new(false),
            force_lowercase,
            evaluation: None,
        }
    }

    /// Update this input's prompt text.
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
    }

    /// Specify whether this input should force its value to be lowercase.
    pub fn set_force_lowercase(&mut self, force: bool) {
        self.force_lowercase = force;
    }

    /// Sets the dependency evaluation which other form elements can react to.
    pub fn set_evaluation(&mut self, evaluation: Evaluation) -> DependencyId {
        let id = DependencyId::new();
        self.evaluation = Some((id, evaluation));
        id
    }
}

impl Control for TextInput {
    fn focusable(&self) -> bool {
        true
    }

    fn update(&mut self, input: KeyEvent) {
        match input.code {
            KeyCode::Char(mut ch) => {
                if self.force_lowercase {
                    ch = ch.to_lowercase().next().unwrap();
                }

                self.text.handle_input(Key::Char(ch));
            }
            KeyCode::Backspace => self.text.handle_input(Key::Backspace),
            KeyCode::Left => self.text.handle_input(Key::Left),
            KeyCode::Right => self.text.handle_input(Key::Right),
            _ => {}
        };
    }

    fn help(&self) -> Option<Segment> {
        Some(Text::new_styled(self.prompt.clone(), help_style()).as_segment())
    }

    fn text(&self) -> (Segment, Option<u16>) {
        let segment = Text::new(self.text.value()).as_segment();
        let cursor_column = self.text.cursor().0 as u16;

        (segment, Some(cursor_column))
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn evaluation(&self) -> Option<(DependencyId, Evaluation)> {
        self.evaluation.clone()
    }

    fn dependency(&self) -> Option<(DependencyId, Action)> {
        None
    }

    fn evaluate(&self, evaluation: &Evaluation) -> bool {
        match evaluation {
            Evaluation::Equals(value) => &self.text.value() == value,
            Evaluation::IsEmpty => self.text.value().is_empty(),
        }
    }

    fn add_to(self, step: &mut CompoundStep) {
        step.add_control(Box::new(self))
    }
}
