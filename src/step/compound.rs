use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{pos, Interface, Position};

use crate::{
    control::Control,
    dependency::{Action, DependencyState},
    style::{error_style, muted_style},
    text::{
        get_segment_length, set_segment_style, set_segment_subset_style, DrawerContents, Segment,
        Text,
    },
    utility::render_segment,
    Form,
};

use super::{InputResult, Step};

/// A single-line step which controls multple controls including static and input elements.
///
/// # Examples
/// ```
/// use tty_form::{
///     Form,
///     step::{Step, CompoundStep},
///     control::{Control, StaticText, TextInput},
/// };
///
/// let mut form = Form::new();
///
/// let mut step = CompoundStep::new();
/// StaticText::new("Welcome, ").add_to(&mut step);
/// TextInput::new("Enter your name:", false).add_to(&mut step);
/// step.add_to(&mut form);
/// ```
pub struct CompoundStep {
    index: Option<usize>,
    controls: Vec<Box<dyn Control>>,
    max_line_length: Option<u16>,
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
    pub fn set_max_line_length(&mut self, max_length: u16) {
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

        // Advance the max_control past unfocusable controls
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

        // Advance to the first focusable control, since the first might be a static element
        if !self.controls[0].focusable() {
            self.advance_control();
        }

        // Register any evaluations in state for this step
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

            // If this is the focused control, let it drive the overall cursor position
            if control_index == self.active_control {
                if let Some(offset) = cursor_offset {
                    cursor_position = Some(pos!(position.x() + offset, position.y()));
                }
            }

            // Resolve this control's dependency and update rendering accordingly
            let mut should_hide = false;
            if let Some((id, action)) = control.dependency() {
                let control_touched = control_index <= self.max_control;
                let evaluation_result = dependency_state.get_evaluation(&id);

                match action {
                    Action::Hide => {
                        if control_touched && evaluation_result {
                            // Determine if the control's dependency source is focused
                            let (step_index, control_index) = dependency_state.get_source(&id);
                            let source_is_focused = step_index == self.index.unwrap()
                                && control_index == self.active_control;

                            // Either render this control muted or hide it, depending on focus
                            if source_is_focused {
                                set_segment_style(&mut segment, muted_style());
                            } else {
                                should_hide = true;
                            }
                        }
                    }
                    Action::Show => should_hide = !evaluation_result,
                }
            }

            // If this step is too-long, render the tail as an error
            if let Some(max_length) = self.max_line_length {
                let segment_length = get_segment_length(&segment) as u16;
                if position.x() + segment_length > max_length {
                    let error_starts_at = max_length - position.x();
                    set_segment_subset_style(
                        &mut segment,
                        error_starts_at.into(),
                        segment_length.into(),
                        error_style(),
                    );
                }
            }

            if !should_hide {
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
        match input.code {
            KeyCode::Enter | KeyCode::Tab => {
                if self.advance_control() {
                    return Some(InputResult::AdvanceForm);
                }
            }
            KeyCode::Esc | KeyCode::BackTab => {
                if self.retreat_control() {
                    return Some(InputResult::RetreatForm);
                }
            }
            _ => {
                let control = &mut self.controls[self.active_control];
                control.update(input);

                // If this control has an evaluation, update its dependency state
                if let Some((id, evaluation)) = control.evaluation() {
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
            segments
                .iter()
                .for_each(|text| result.push_str(text.content()));
        }

        result.push('\n');

        result
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}
