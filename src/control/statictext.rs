use crossterm::event::KeyEvent;
use tty_interface::Style;

use crate::{
    dependency::{Action, DependencyId, Evaluation},
    step::CompoundStep,
    text::{DrawerContents, Segment, Text},
};

use super::Control;

/// Static, unfocusable, formatable display text. May be dependent on other form elements.
///
/// # Examples
/// ```
/// use tty_interface::Style;
///
/// use tty_form::{
///     step::CompoundStep,
///     control::{Control, StaticText},
/// };
///
/// let mut text = StaticText::new("Hello, world!");
/// text.set_style(Style::new().set_bold(true));
///
/// let mut step = CompoundStep::new();
/// text.add_to(&mut step);
/// ```
pub struct StaticText {
    text: String,
    style: Option<Style>,
    dependency: Option<(DependencyId, Action)>,
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
        (Text::new(self.text.to_string()).as_segment(), None)
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
