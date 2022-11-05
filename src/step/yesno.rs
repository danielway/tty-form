use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{pos, Interface, Position};
use tty_text::Key;

use crate::{
    dependency::{DependencyId, DependencyState, Evaluation},
    style::{help_style, muted_style},
    text::{DrawerContents, Segment, Text},
    Form,
};

use super::{InputResult, Step};

/// A boolean input which, if true, accepts a text description.
pub struct YesNoStep {
    prompt: String,
    prefix: String,
    omit_if_no: bool,
    toggle_value: bool,
    text_prompt: String,
    text: tty_text::Text,
    evaluation: Option<(DependencyId, Evaluation)>,
}

impl YesNoStep {
    pub fn new(prompt: &str, description_prompt: &str, prefix: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            prefix: prefix.to_string(),
            omit_if_no: true,
            toggle_value: false,
            text_prompt: description_prompt.to_string(),
            text: tty_text::Text::new(false),
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

    fn get_display_value(&self) -> String {
        if !self.text.value().is_empty() {
            self.text.value()
        } else if self.toggle_value {
            String::from("Yes")
        } else {
            String::from("No")
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
        if self.toggle_value || is_focused || !self.omit_if_no {
            let display_value = self.get_display_value();
            if !self.toggle_value && (is_focused || !self.omit_if_no) {
                // Render muted prompt and value
                interface.set_styled(
                    position,
                    &format!("{}: {}", self.prefix, display_value),
                    muted_style(),
                );
            } else if is_focused && self.toggle_value && self.text.value().is_empty() {
                // Render a white prefix with muted value
                interface.set(position, &format!("{}:", self.prefix));

                let value_position = pos!(self.prefix.len() as u16 + 2, position.y());
                interface.set_styled(value_position, &display_value, muted_style());
            } else if is_focused || self.toggle_value {
                // Render white prompt and value
                interface.set(position, &format!("{}: {}", self.prefix, display_value));
            }

            if is_focused && self.toggle_value {
                let (cursor_column, _) = self.text.cursor();
                let cursor = pos!((self.prefix.len() + 2 + cursor_column) as u16, position.y());
                interface.set_cursor(Some(cursor));
            }

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
            KeyCode::Esc | KeyCode::BackTab => return Some(InputResult::RetreatForm),
            KeyCode::Enter | KeyCode::Tab => return Some(InputResult::AdvanceForm),
            _ => {}
        };

        if self.text.value().is_empty()
            && (input.code == KeyCode::Up || input.code == KeyCode::Down)
        {
            self.toggle_value = !self.toggle_value;
        }

        if self.toggle_value {
            match input.code {
                KeyCode::Char(ch) => self.text.handle_input(Key::Char(ch)),
                KeyCode::Backspace => self.text.handle_input(Key::Backspace),
                KeyCode::Left => self.text.handle_input(Key::Left),
                KeyCode::Right => self.text.handle_input(Key::Right),
                _ => {}
            };
        }

        if let Some((id, evaluation)) = &self.evaluation {
            let value = match evaluation {
                Evaluation::Equal(value) => value == &self.get_display_value(),
                Evaluation::NotEqual(value) => value != &self.get_display_value(),
                Evaluation::IsEmpty => false,
            };

            dependency_state.update_evaluation(&id, value);
        }

        None
    }

    fn help(&self) -> Segment {
        Text::new_styled(
            if self.toggle_value {
                self.text_prompt.to_string()
            } else {
                self.prompt.to_string()
            },
            help_style(),
        )
        .as_segment()
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn result(&self, _dependency_state: &DependencyState) -> String {
        if self.omit_if_no && !self.toggle_value {
            return String::new();
        }

        format!("{}: {}\n", self.prefix, self.get_display_value())
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}
