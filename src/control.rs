use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::Style;
use tty_text::Key;

use crate::{Action, CompoundStep, DependencyId, DrawerContents, Evaluation, Segment, Text};

/// An element of a [CompoundStep] which may be a focusable input.
pub trait Control {
    /// Whether this control is a focusable input.
    fn focusable(&self) -> bool;

    /// Updates the control's state from the given input event.
    fn update(&mut self, input: KeyEvent);

    /// This control's descriptive help text, if available.
    fn help(&self) -> Option<Segment>;

    /// This control's rendered contents and an optional offset for the cursor.
    fn text(&self) -> (Segment, Option<u16>);

    /// This control's drawer contents, if available.
    fn drawer(&self) -> Option<DrawerContents>;

    /// This control's dependency evaluation which other controls may react to.
    fn evaluation(&self) -> Option<(DependencyId, Evaluation)>;

    /// This control's dependency which it may react to.
    fn dependency(&self) -> Option<(DependencyId, Action)>;

    /// Perform an evaluation against this control's current state.
    fn evaluate(&self, evaluation: &Evaluation) -> bool;

    /// Finish configuration and add this control to the specified form step.
    fn add_to(self, step: &mut CompoundStep);
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
/// text.add_to(&mut step);
/// ```
pub struct StaticText {
    text: String,
    style: Option<Style>,
    dependency: Option<(DependencyId, Action)>,
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
            dependency: None,
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

    /// Sets a dependency on the specified ID, performing some action if it evaluates true.
    pub fn set_dependency(&mut self, id: DependencyId, action: Action) {
        self.dependency = Some((id, action));
    }
}

impl Control for StaticText {
    fn focusable(&self) -> bool {
        false
    }

    fn update(&mut self, _input: KeyEvent) {}

    fn help(&self) -> Option<Segment> {
        None
    }

    fn text(&self) -> (Segment, Option<u16>) {
        (Text::new(self.text.clone()).as_segment(), None)
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn evaluation(&self) -> Option<(DependencyId, Evaluation)> {
        None
    }

    fn dependency(&self) -> Option<(DependencyId, Action)> {
        self.dependency.clone()
    }

    fn evaluate(&self, _evaluation: &Evaluation) -> bool {
        false
    }

    fn add_to(self, step: &mut CompoundStep) {
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
/// TextInput::new("Enter your name:", false).add_to(&mut step);
/// ```
pub struct TextInput {
    prompt: String,
    text: tty_text::Text,
    force_lowercase: bool,
    evaluation: Option<(DependencyId, Evaluation)>,
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
        Some(Text::new(self.prompt.clone()).as_segment())
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
        Some(Text::new(self.prompt.clone()).as_segment())
    }

    fn text(&self) -> (Segment, Option<u16>) {
        let value = &self.options[self.selected_option].value;
        let segment = Text::new(value.clone()).as_segment();

        (segment, Some(0))
    }

    fn drawer(&self) -> Option<DrawerContents> {
        let mut items = Vec::new();

        for option in &self.options {
            items
                .push(Text::new(format!("{} - {}", option.value, option.description)).as_segment());
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
