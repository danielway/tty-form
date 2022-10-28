use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    dependency::{Action, DependencyId, Evaluation},
    step::compound::CompoundStep,
    style::{drawer_selected_style, drawer_style, help_style},
    text::{DrawerContents, Segment, Text},
};

use super::Control;

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
/// ]).add_to(&mut step);
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
    fn focusable(&self) -> bool {
        true
    }

    fn update(&mut self, input: KeyEvent) {
        match input.code {
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

    fn help(&self) -> Option<Segment> {
        Some(Text::new_styled(self.prompt.clone(), help_style()).as_segment())
    }

    fn text(&self) -> (Segment, Option<u16>) {
        let value = &self.options[self.selected_option].value;
        let segment = Text::new(value.clone()).as_segment();

        (segment, Some(0))
    }

    fn drawer(&self) -> Option<DrawerContents> {
        let mut items = Vec::new();

        for (option_index, option) in self.options.iter().enumerate() {
            let mut text = format!("   {} - {}", option.value, option.description);
            let mut style = drawer_style();

            if option_index == self.selected_option {
                style = drawer_selected_style();
                text.replace_range(1..2, ">");
            }

            items.push(Text::new_styled(text, style).as_segment());
        }

        Some(items)
    }

    fn evaluation(&self) -> Option<(DependencyId, Evaluation)> {
        None
    }

    fn dependency(&self) -> Option<(DependencyId, Action)> {
        None
    }

    fn evaluate(&self, evaluation: &Evaluation) -> bool {
        match evaluation {
            Evaluation::Equals(value) => &self.options[self.selected_option].value == value,
            Evaluation::IsEmpty => false,
        }
    }

    fn add_to(self, step: &mut CompoundStep) {
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
