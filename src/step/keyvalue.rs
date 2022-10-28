use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{pos, Interface, Position};
use tty_text::Key;

use crate::{
    dependency::{DependencyId, DependencyState, Evaluation},
    style::drawer_style,
    text::{DrawerContents, Segment, Text},
    Form,
};

use super::{InputResult, Step};

/// A key-value-pair set entry step.
pub struct KeyValueStep {
    prompt: String,
    pairs: Vec<(tty_text::Text, tty_text::Text)>,
    active_pair: usize,
    active_field: usize,
    evaluation: Option<(DependencyId, Evaluation)>,
}

impl KeyValueStep {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            pairs: vec![(tty_text::Text::new(false), tty_text::Text::new(false))],
            active_pair: 0,
            active_field: 0,
            evaluation: None,
        }
    }

    pub fn set_evaluation(&mut self, evaluation: Evaluation) -> DependencyId {
        let id = DependencyId::new();
        self.evaluation = Some((id, evaluation));
        id
    }
}

impl Step for KeyValueStep {
    fn initialize(&mut self, _dependency_state: &mut DependencyState, _index: usize) {}

    fn render(
        &self,
        interface: &mut Interface,
        _dependency_state: &DependencyState,
        mut position: Position,
        _is_focused: bool,
    ) -> u16 {
        for (pair_index, (key, value)) in self.pairs.iter().enumerate() {
            interface.set(position, &format!("{}: {}", key.value(), value.value()));

            if pair_index == self.active_pair {
                interface.set_cursor(Some(position));
            }

            position = pos!(position.x(), position.y() + 1);
        }

        self.pairs.len() as u16
    }

    fn update(
        &mut self,
        _dependency_state: &mut DependencyState,
        input: KeyEvent,
    ) -> Option<InputResult> {
        let text = if self.active_field == 0 {
            &mut self.pairs[self.active_pair].0
        } else {
            &mut self.pairs[self.active_pair].1
        };

        match input.code {
            KeyCode::Char(ch) => text.handle_input(Key::Char(ch)),
            KeyCode::Backspace => text.handle_input(Key::Backspace),
            KeyCode::Left => text.handle_input(Key::Left),
            KeyCode::Right => text.handle_input(Key::Right),
            _ => {}
        };

        None
    }

    fn help(&self) -> Segment {
        Text::new_styled(self.prompt.to_string(), drawer_style()).as_segment()
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn result(&self, _dependency_state: &DependencyState) -> String {
        String::new() // TODO
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}
