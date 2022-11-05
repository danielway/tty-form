use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{Interface, Position, Style};

use crate::{
    dependency::{DependencyId, DependencyState, Evaluation},
    style::{help_style, muted_style},
    text::{DrawerContents, Segment, Text},
    Form,
};

use super::{InputResult, Step};

pub struct YesNoStep {
    prompt: String,
    prefix: String,
    omit_if_no: bool,
    value: bool,
    evaluation: Option<(DependencyId, Evaluation)>,
}

impl YesNoStep {
    pub fn new(prompt: &str, prefix: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            prefix: prefix.to_string(),
            omit_if_no: true,
            value: false,
            evaluation: None,
        }
    }

    pub fn set_omit_if_no(&mut self, omit: bool) {
        self.omit_if_no = omit;
    }

    pub fn set_evaluation(&mut self, evaluation: Evaluation) -> DependencyId {
        let id = DependencyId::new();
        self.evaluation = Some((id, evaluation));
        id
    }

    fn value_as_string(&self) -> &str {
        if self.value {
            "Yes"
        } else {
            "No"
        }
    }
}

impl Step for YesNoStep {
    fn initialize(&mut self, _dependency_state: &mut DependencyState, _index: usize) {}

    fn render(
        &self,
        interface: &mut Interface,
        _dependency_state: &DependencyState,
        position: Position,
        is_focused: bool,
    ) -> u16 {
        if self.value || is_focused || !self.omit_if_no {
            let style = if is_focused && self.omit_if_no && !self.value {
                muted_style()
            } else {
                Style::new()
            };

            if is_focused {
                interface.set_cursor(Some(position));
            }

            interface.set_styled(
                position,
                &format!("{}: {}", self.prefix, self.value_as_string()),
                style,
            );

            return 1;
        }

        0
    }

    fn update(
        &mut self,
        dependency_state: &mut DependencyState,
        input: KeyEvent,
    ) -> Option<InputResult> {
        match input.code {
            KeyCode::Enter | KeyCode::Tab => return Some(InputResult::AdvanceForm),
            KeyCode::Esc => return Some(InputResult::RetreatForm),
            KeyCode::Up | KeyCode::Down => self.value = !self.value,
            _ => {}
        };

        if let Some((id, evaluation)) = &self.evaluation {
            let value = match evaluation {
                Evaluation::Equals(value) => value == self.value_as_string(),
                Evaluation::IsEmpty => false,
            };

            dependency_state.update_evaluation(&id, value);
        }

        None
    }

    fn help(&self) -> Segment {
        Text::new_styled(self.prompt.to_string(), help_style()).as_segment()
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn result(&self, _dependency_state: &DependencyState) -> String {
        if self.omit_if_no && !self.value {
            return String::new();
        }

        format!("{}: {}", self.prefix, self.value_as_string())
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}
