use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{pos, Color, Interface, Position, Style};
use tty_text::Key;

use crate::{
    dependency::DependencyState, render_segment, set_segment_style, Action, Control,
    DrawerContents, Form, Segment, Text,
};

/// A distinct, vertically-separated phase of the form.
pub trait Step {
    /// Perform any post-configuration initialization actions for this step.
    fn initialize(&mut self, dependency_state: &mut DependencyState, index: usize);

    /// Render this step at the specified position and return the height of the rendered content.
    fn render(
        &self,
        interface: &mut Interface,
        dependency_state: &DependencyState,
        position: Position,
        is_focused: bool,
    ) -> u16;

    /// Handle the specified input event, optionally returning an instruction for the form.
    fn update(
        &mut self,
        dependency_state: &mut DependencyState,
        input: KeyEvent,
    ) -> Option<InputResult>;

    /// Retrieve this step's current help text.
    fn help(&self) -> Segment;

    /// Retrieve this step's current drawer contents, if applicable.
    fn drawer(&self) -> Option<DrawerContents>;

    /// Retrieves this step's final WYSIWYG result.
    fn result(&self, dependency_state: &DependencyState) -> String;

    /// Complete configuration and add this step to the form.
    fn add_to(self, form: &mut Form);
}

/// After processing an input event, an action may be returned to the form from the step.
pub enum InputResult {
    /// Advance the form to the next step.
    AdvanceForm,
    /// Retreat the form to the previous step.
    RetreatForm,
}

/// A single-line step which controls multple controls including static and input elements.
///
/// # Examples
/// ```
/// use tty_form::{Form, Step, CompoundStep, Control, StaticText, TextInput};
///
/// let mut form = Form::new();
///
/// let mut step = CompoundStep::new();
/// StaticText::new("A Form - ").add_to(&mut step);
/// TextInput::new("Enter your name:", false).add_to(&mut step);
/// step.add_to(&mut form);
/// ```
pub struct CompoundStep {
    index: Option<usize>,
    controls: Vec<Box<dyn Control>>,
    max_line_length: Option<u8>,
    active_control: usize,
    max_control: usize,
}

impl CompoundStep {
    /// Create a new compound step with no controls.
    pub fn new() -> Self {
        Self {
            index: None,
            controls: Vec::new(),
            max_line_length: None,
            active_control: 0,
            max_control: 0,
        }
    }

    /// Append the specified control to this step.
    pub fn add_control(&mut self, control: Box<dyn Control>) {
        self.controls.push(control);
    }

    /// Set this step's maximum total line length.
    pub fn set_max_line_length(&mut self, max_length: u8) {
        self.max_line_length = Some(max_length);
    }

    /// Advance the step's state to the next control. Returns true if we've reached the end of this
    /// step and the form should advance to the next.
    fn advance_control(&mut self) -> bool {
        let mut reached_last_control = false;
        loop {
            if self.active_control + 1 >= self.controls.len() {
                reached_last_control = true;
                break;
            }

            self.active_control += 1;

            if self.controls[self.active_control].focusable() {
                break;
            }
        }

        if self.active_control > self.max_control {
            self.max_control = self.active_control;
            loop {
                if self.max_control + 1 >= self.controls.len() {
                    break;
                }

                if !self.controls[self.max_control + 1].focusable() {
                    self.max_control += 1;
                } else {
                    break;
                }
            }
        }

        reached_last_control
    }

    /// Retreat the step's state to the previous control. Returns true if we've reached the start
    /// of this step and the form should retreat to the previous.
    fn retreat_control(&mut self) -> bool {
        loop {
            if self.active_control == 0 {
                return true;
            }

            self.active_control -= 1;

            if self.controls[self.active_control].focusable() {
                break;
            }
        }

        false
    }
}

impl Step for CompoundStep {
    fn initialize(&mut self, dependency_state: &mut DependencyState, index: usize) {
        self.index = Some(index);

        if !self.controls[0].focusable() {
            self.advance_control();
        }

        for (control_index, control) in self.controls.iter().enumerate() {
            for (id, evaluation) in control.evaluation() {
                dependency_state.register_evaluation(&id, index, control_index);

                let value = control.evaluate(&evaluation);
                dependency_state.update_evaluation(&id, value);
            }
        }
    }

    fn render(
        &self,
        interface: &mut Interface,
        dependency_state: &DependencyState,
        mut position: Position,
        is_focused: bool,
    ) -> u16 {
        interface.clear_line(position.y());

        let mut cursor_position = None;
        for (control_index, control) in self.controls.iter().enumerate() {
            let (mut segment, cursor_offset) = control.text();

            if control_index == self.active_control {
                if let Some(offset) = cursor_offset {
                    cursor_position = Some(pos!(position.x() + offset, position.y()));
                }
            }

            let mut should_hide = false;
            if let Some((id, action)) = control.dependency() {
                if dependency_state.get_evaluation(&id) {
                    if action == Action::Hide {
                        let (step_index, control_index) = dependency_state.get_source(&id);
                        if step_index == self.index.unwrap() && control_index == self.active_control
                        {
                            let total_length = segment.iter().map(|s| s.content().len()).sum();
                            set_segment_style(
                                &mut segment,
                                0,
                                total_length,
                                Style::default().set_foreground(Color::DarkGrey),
                            );
                        } else {
                            should_hide = true;
                        }
                    }
                } else if action == Action::Show {
                    should_hide = true;
                }
            }

            if control_index >= self.max_control || !should_hide {
                position = render_segment(interface, position, segment);
            }
        }

        if is_focused {
            interface.set_cursor(cursor_position);
        }

        1
    }

    fn update(
        &mut self,
        dependency_state: &mut DependencyState,
        input: KeyEvent,
    ) -> Option<InputResult> {
        match (input.modifiers, input.code) {
            (_, KeyCode::Enter) => {
                if self.advance_control() {
                    return Some(InputResult::AdvanceForm);
                }
            }
            (_, KeyCode::Esc) => {
                if self.retreat_control() {
                    return Some(InputResult::RetreatForm);
                }
            }
            _ => {
                let control = &mut self.controls[self.active_control];

                control.update(input);
                for (id, evaluation) in control.evaluation() {
                    let value = control.evaluate(&evaluation);
                    dependency_state.update_evaluation(&id, value);
                }
            }
        }

        None
    }

    fn help(&self) -> Segment {
        self.controls[self.active_control]
            .help()
            .unwrap_or(Text::new(String::new()).as_segment())
    }

    fn drawer(&self) -> Option<DrawerContents> {
        self.controls[self.active_control].drawer()
    }

    fn result(&self, _dependency_state: &DependencyState) -> String {
        let mut result = String::new();

        for control in &self.controls {
            let (segments, _) = control.text();
            for text in segments {
                result.push_str(text.content());
            }
        }

        result.push('\n');

        result
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}

/// A multi-line text input step.
///
/// # Examples
/// ```
/// use tty_form::{Form, Step, TextBlockStep};
///
/// let mut form = Form::new();
///
/// let mut step = TextBlockStep::new("Enter your story:");
/// step.set_max_line_length(100);
/// step.add_to(&mut form);
/// ```
pub struct TextBlockStep {
    prompt: String,
    text: tty_text::Text,
    max_line_length: Option<u8>,
}

impl TextBlockStep {
    /// Create a new, default text block step.
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            text: tty_text::Text::new(true),
            max_line_length: None,
        }
    }

    /// Set this text block step's optional maximum line grapheme length.
    pub fn set_max_line_length(&mut self, max_length: u8) {
        self.max_line_length = Some(max_length);
    }
}

impl Step for TextBlockStep {
    fn initialize(&mut self, _dependency_state: &mut DependencyState, _index: usize) {}

    fn render(
        &self,
        interface: &mut Interface,
        _dependency_state: &DependencyState,
        position: Position,
        is_focused: bool,
    ) -> u16 {
        let lines = self.text.lines();
        for (line_index, line) in lines.iter().enumerate() {
            interface.set(pos!(0, position.y() + line_index as u16), line);
        }

        if is_focused {
            let cursor = self.text.cursor();
            let (x, y) = (cursor.0 as u16, cursor.1 as u16);
            interface.set_cursor(Some(pos!(x, y + position.y())));
        }

        lines.len() as u16
    }

    fn update(
        &mut self,
        _dependency_state: &mut DependencyState,
        input: KeyEvent,
    ) -> Option<InputResult> {
        if input.code == KeyCode::Enter {
            let mut last_two_empty = self.text.lines().iter().count() > 2;
            if last_two_empty {
                for line in self.text.lines().iter().rev().take(2) {
                    if !line.is_empty() {
                        last_two_empty = false;
                    }
                }
            }

            if last_two_empty {
                return Some(InputResult::AdvanceForm);
            }
        }

        if input.code == KeyCode::Esc {
            return Some(InputResult::RetreatForm);
        }

        match input.code {
            KeyCode::Enter => self.text.handle_input(Key::Enter),
            KeyCode::Char(ch) => self.text.handle_input(Key::Char(ch)),
            KeyCode::Backspace => self.text.handle_input(Key::Backspace),
            KeyCode::Up => self.text.handle_input(Key::Up),
            KeyCode::Down => self.text.handle_input(Key::Down),
            KeyCode::Left => self.text.handle_input(Key::Left),
            KeyCode::Right => self.text.handle_input(Key::Right),
            _ => {}
        };

        None
    }

    fn help(&self) -> Segment {
        Text::new(self.prompt.to_string()).as_segment()
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn result(&self, _dependency_state: &DependencyState) -> String {
        let mut result = self.text.value();
        result.push('\n');
        result
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}
