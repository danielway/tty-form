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
    force_lowercase: bool,
}

impl TextInput {
    /// Create a new, default text input.
    pub(crate) fn new() -> Self {
        Self {
            force_lowercase: false,
        }
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
    options: Vec<String>,
}

impl SelectInput {
    /// Create a new option input with no items.
    pub(crate) fn new() -> Self {
        Self {
            options: Vec::new(),
        }
    }

    /// Add an option to this input's list.
    pub fn add_option(&mut self, option: String) {
        self.options.push(option);
    }

    /// Set this input's options.
    pub fn set_options(&mut self, options: Vec<String>) {
        self.options = options;
    }
}
