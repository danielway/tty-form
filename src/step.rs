use tty_interface::Style;

use crate::{Control, SelectInput, StaticText, TextInput};

/// A distinct, vertically-separated phase of the form.
pub enum Step {
    /// A multi-control single-line input step.
    Compound(CompoundStep),
    /// A multi-line text block input step.
    TextBlock(TextBlockStep),
    /// An unfocusable, vertically-separated text description in the form.
    Description(DescriptionStep),
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
}

impl CompoundStep {
    /// Create a new compound step with no controls.
    pub(crate) fn new() -> Self {
        Self {
            controls: Vec::new(),
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
    pub(crate) fn new() -> Self {
        Self {
            max_line_length: None,
        }
    }

    /// Set this text block step's optional maximum line grapheme length.
    pub fn set_max_line_length(&mut self, max_length: u8) {
        self.max_line_length = Some(max_length);
    }
}

/// An unfocusable, vertically-separated text description in the form.
///
/// # Examples
/// ```
/// use tty_form::Form;
/// use tty_interface::{Style, Color};
///
/// let mut form = Form::default();
/// let mut step = form.add_description_step();
/// step.set_text("Here's a description of this phase of the form.");
/// step.set_style(Style::default().set_foreground(Color::Red));
/// ```
pub struct DescriptionStep {
    text: String,
    style: Option<Style>,
}

impl DescriptionStep {
    pub(crate) fn new() -> Self {
        Self {
            text: String::new(),
            style: None,
        }
    }

    /// Set this description's text.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Set this description's optional styling.
    pub fn set_style(&mut self, style: Style) {
        self.style = Some(style);
    }
}
