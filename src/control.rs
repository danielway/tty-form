use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::Style;
use tty_text::{Key, Text};

use crate::CompoundStep;

/// An element of a [CompoundStep] which may be a focusable input.
pub trait Control {
    /// Whether this control is a focusable input.
    fn is_focusable(&self) -> bool;

    /// Updates the control's state from the given input event.
    fn handle_input(&mut self, key_event: KeyEvent);

    /// Get this control's descriptive help text, if available.
    fn get_help(&self) -> Option<String>;

    /// Get this control's rendered result contents.
    fn get_text(&self) -> (String, Option<u16>);

    /// Get this control's drawer contents, if available.
    fn get_drawer(&self) -> Option<Vec<String>>;

    /// Finish configuration and add this control to the specified form step.
    fn add_to_step(self, step: &mut CompoundStep);
}

/// Static, unfocusable display text. May be formatted.
///
/// # Examples
/// ```
/// use tty_form::{CompoundStep, Control, StaticText};
/// use tty_interface::Style;
///
/// let mut text = StaticText::new("Hello, world!");
/// text.set_style(Style::default().set_bold(true));
///
/// let mut step = CompoundStep::new();
/// text.add_to_step(&mut step);
/// ```
pub struct StaticText {
    text: String,
    style: Option<Style>,
}

impl Default for StaticText {
    /// Create a default static text control with no contents.
    fn default() -> Self {
        Self::new("")
    }
}

impl StaticText {
    /// Create a new static text control with the specified content.
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
/// use tty_form::{CompoundStep, Control, TextInput};
///
/// let mut step = CompoundStep::new();
/// TextInput::new("Enter your name:", false)
///     .add_to_step(&mut step);
/// ```
pub struct TextInput {
    prompt: String,
    text: Text,
    force_lowercase: bool,
}

impl Default for TextInput {
    /// Create a default text input with no prompt which allows mixed cases.
    fn default() -> Self {
        Self::new("", false)
    }
}

impl TextInput {
    /// Create a new text input control with the specified prompt and casing-rules.
    pub fn new(prompt: &str, force_lowercase: bool) -> Self {
        Self {
            prompt: prompt.to_string(),
            text: Text::new(false),
            force_lowercase,
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
        }
    }

    fn get_help(&self) -> Option<String> {
        Some(self.prompt.clone())
    }

    fn get_text(&self) -> (String, Option<u16>) {
        (self.text.value(), Some(self.text.cursor().0 as u16))
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
/// use tty_form::{CompoundStep, Control, SelectInput};
/// use tty_interface::Style;
///
/// let mut step = CompoundStep::new();
/// SelectInput::new("Select favorite food:", vec![
///     ("Pizza", "A supreme pizza."),
///     ("Burgers", "A hamburger with cheese."),
///     ("Fries", "Simple potato french-fries."),
/// ]).add_to_step(&mut step);
/// ```
pub struct SelectInput {
    prompt: String,
    options: Vec<SelectInputOption>,
    selected_option: usize,
}

impl Default for SelectInput {
    /// Create a new option-selection input with no options.
    fn default() -> Self {
        Self::new("", Vec::new())
    }
}

impl SelectInput {
    /// Create a new option-selection input with the specified prompt and options.
    pub fn new(prompt: &str, options: Vec<(&str, &str)>) -> Self {
        Self {
            prompt: prompt.to_string(),
            options: options
                .iter()
                .map(|(value, description)| SelectInputOption::new(value, description))
                .collect(),
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
            _ => {}
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
