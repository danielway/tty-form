use crossterm::event::{KeyEvent, KeyCode};
use tty_interface::Style;

use crate::CompoundStep;

/// An element of a CompoundStep which may be a focusable input.
pub trait Control {
    fn is_focusable(&self) -> bool;
    fn handle_input(&mut self, key_event: KeyEvent);
    fn get_help(&self) -> Option<String>;
    fn get_text(&self) -> (String, Option<u16>);
    fn get_drawer(&self) -> Option<Vec<String>>;
    fn add_to_step(self, step: &mut CompoundStep);
}

/// Static, unfocusable display text. May be formatted.
///
/// # Examples
/// ```
/// use tty_form::Form;
/// use tty_interface::Style;
///
/// let mut form = Form::default();
/// let mut text = form.add_compound_step().add_static_text();
///
/// text.set_text("Hello, world!");
/// text.set_style(Style::default().set_bold(true));
/// ```
pub struct StaticText {
    text: String,
    style: Option<Style>,
}

impl Default for StaticText {
    fn default() -> Self {
        Self::new("")
    }
}

impl StaticText {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: None,
        }
    }

    /// Set the text for this control.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Set the optional style for this control.
    pub fn set_style(&mut self, style: Style) {
        self.style = Some(style);
    }
}

impl Control for StaticText {
    fn is_focusable(&self) -> bool {
        false
    }

    fn handle_input(&mut self, _key_event: KeyEvent) {}

    fn get_help(&self) -> Option<String> {
        None
    }

    fn get_text(&self) -> (String, Option<u16>) {
        (self.text.clone(), None)
    }

    fn get_drawer(&self) -> Option<Vec<String>> {
        None
    }

    fn add_to_step(self, step: &mut CompoundStep) {
        step.add_control(Box::new(self));
    }
}

/// A text field input.
///
/// # Examples
/// ```
/// use tty_form::Form;
///
/// let mut form = Form::default();
/// let mut input = form.add_compound_step().add_text_input();
///
/// input.set_force_lowercase(true);
/// ```
pub struct TextInput {
    prompt: String,
    value: String,
    force_lowercase: bool,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new("")
    }
}

impl TextInput {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            value: String::new(),
            force_lowercase: false,
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
}

impl Control for TextInput {
    fn is_focusable(&self) -> bool {
        true
    }

    fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(ch) => self.value.push(ch),
            _ => {},
        }
    }

    fn get_help(&self) -> Option<String> {
        Some(self.prompt.clone())
    }

    fn get_text(&self) -> (String, Option<u16>) {
        (self.value.clone(), Some(self.value.len() as u16))
    }

    fn get_drawer(&self) -> Option<Vec<String>> {
        None
    }

    fn add_to_step(self, step: &mut CompoundStep) {
        step.add_control(Box::new(self))
    }
}

/// An option selection field.
///
/// # Examples
/// ```
/// use tty_form::Form;
///
/// let mut form = Form::default();
/// let mut input = form.add_compound_step().add_text_input();
///
/// input.set_force_lowercase(true);
/// ```
pub struct SelectInput {
    prompt: String,
    options: Vec<SelectInputOption>,
    selected_option: usize,
}

impl Default for SelectInput {
    fn default() -> Self {
        Self::new("", Vec::new())
    }
}

impl SelectInput {
    pub fn new(prompt: &str, options: Vec<(&str, &str)>) -> Self {
        Self {
            prompt: prompt.to_string(),
            options: options.iter().map(|(value, description)| SelectInputOption::new(value, description)).collect(),
            selected_option: 0,
        }
    }

    /// Update this input's prompt text.
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
    }

    /// Add an option to this input's list.
    pub fn add_option(&mut self, option: SelectInputOption) {
        self.options.push(option);
    }

    /// Set this input's options.
    pub fn set_options(&mut self, options: Vec<SelectInputOption>) {
        self.options = options;
    }
}

impl Control for SelectInput {
    fn is_focusable(&self) -> bool {
        true
    }

    fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => {
                if self.selected_option == 0 {
                    self.selected_option = self.options.len() - 1;
                } else {
                    self.selected_option -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_option + 1 == self.options.len() {
                    self.selected_option = 0;
                } else {
                    self.selected_option += 1;
                }
            }
            _ => {},
        }
    }

    fn get_help(&self) -> Option<String> {
        Some(self.prompt.clone())
    }

    fn get_text(&self) -> (String, Option<u16>) {
        (self.options[self.selected_option].value.clone(), Some(0))
    }

    fn get_drawer(&self) -> Option<Vec<String>> {
        let mut items = Vec::new();

        for option in &self.options {
            items.push(format!("{} - {}", option.value, option.description));
        }

        Some(items)
    }

    fn add_to_step(self, step: &mut CompoundStep) {
        step.add_control(Box::new(self))
    }
}

/// A option for an option selection input.
pub struct SelectInputOption {
    value: String,
    description: String,
}

impl SelectInputOption {
    /// Create a new option with the specified value and description.
    pub fn new(value: &str, description: &str) -> Self {
        Self {
            value: value.to_string(),
            description: description.to_string(),
        }
    }

    /// This option's value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// This option's descriptive text.
    pub fn description(&self) -> &str {
        &self.description
    }
}
