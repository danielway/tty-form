use tty_interface::Style;

/// An element of a CompoundStep which may be a focusable input.
pub enum Control {
    /// Static, unfocusable display text.
    Static(StaticText),
    /// A text input.
    Text(TextInput),
    /// An option selection input.
    Select(SelectInput),
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

impl StaticText {
    /// Create a new, blank, unstyled text control.
    pub(crate) fn new() -> Self {
        Self {
            text: String::new(),
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
    force_lowercase: bool,
}

impl TextInput {
    /// Create a new, default text input with an empty prompt.
    pub(crate) fn new() -> Self {
        Self {
            prompt: String::new(),
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
}

impl SelectInput {
    /// Create a new option input with no items or prompt.
    pub(crate) fn new() -> Self {
        Self {
            prompt: String::new(),
            options: Vec::new(),
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

/// A option for an option selection input.
pub struct SelectInputOption {
    value: String,
    description: String,
}

impl SelectInputOption {
    /// Create a new option with the specified value and description.
    pub fn new(value: String, description: String) -> Self {
        Self { value, description }
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
