use crossterm::event::{KeyCode, KeyEvent};
use tty_interface::{pos, Interface, Position};
use tty_text::Key;

use crate::{
    dependency::{DependencyId, DependencyState, Evaluation},
    style::help_style,
    text::{DrawerContents, Segment, Text},
    Form,
};

use super::{InputResult, Step};

/// A key-value-pair set entry step.
pub struct KeyValueStep {
    prompt: String,
    pairs: Vec<(tty_text::Text, tty_text::Text)>,
    focused_pair: usize,
    key_focused: bool,
    evaluation: Option<(DependencyId, Evaluation)>,
}

impl KeyValueStep {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            pairs: vec![(tty_text::Text::new(false), tty_text::Text::new(false))],
            focused_pair: 0,
            key_focused: true,
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
        is_focused: bool,
    ) -> u16 {
        for (pair_index, (key, value)) in self.pairs.iter().enumerate() {
            let line = if value.value().is_empty() {
                key.value()
            } else {
                format!("{}: {}", key.value(), value.value())
            };

            interface.set(position, &line);

            if is_focused && pair_index == self.focused_pair {
                let cursor = pos!(
                    if self.key_focused {
                        key.cursor().0
                    } else {
                        key.value().len() + 2 + value.cursor().0
                    } as u16,
                    position.y()
                );

                interface.set_cursor(Some(cursor));
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
        let text = if self.key_focused {
            &mut self.pairs[self.focused_pair].0
        } else {
            &mut self.pairs[self.focused_pair].1
        };

        match input.code {
            KeyCode::Enter | KeyCode::Tab => {
                if text.value().is_empty() {
                    if self.key_focused {
                        self.pairs.remove(self.focused_pair);
                        self.focused_pair -= 1;

                        // Advance past this step
                        return Some(InputResult::AdvanceForm);
                    } else {
                        // Append a new KVP
                        self.key_focused = true;
                        self.focused_pair += 1;
                        if self.focused_pair == self.pairs.len() {
                            self.pairs
                                .push((tty_text::Text::new(false), tty_text::Text::new(false)));
                        }
                    }
                } else {
                    if self.key_focused {
                        // Switch to the value entry
                        self.key_focused = false;
                    } else {
                        // Append a new KVP
                        self.key_focused = true;
                        self.focused_pair += 1;
                        if self.focused_pair == self.pairs.len() {
                            self.pairs
                                .push((tty_text::Text::new(false), tty_text::Text::new(false)));
                        }
                    }
                }
            }
            KeyCode::Esc | KeyCode::BackTab => {
                if !self.key_focused {
                    self.key_focused = true;
                } else {
                    if self.focused_pair > 0 {
                        self.focused_pair -= 1;
                        self.key_focused = false;
                    } else {
                        return Some(InputResult::RetreatForm);
                    }
                }
            }
            KeyCode::Char(ch) => text.handle_input(Key::Char(ch)),
            KeyCode::Backspace => {
                if text.value().is_empty() {
                    if !self.key_focused {
                        self.key_focused = true;
                    } else {
                        if self.focused_pair > 0 {
                            self.pairs.remove(self.focused_pair);
                            self.focused_pair -= 1;
                            self.key_focused = false;
                        } else {
                            return Some(InputResult::RetreatForm);
                        }
                    }
                } else {
                    text.handle_input(Key::Backspace);
                }
            }
            KeyCode::Left => text.handle_input(Key::Left),
            KeyCode::Right => text.handle_input(Key::Right),
            _ => {}
        };

        None
    }

    fn help(&self) -> Segment {
        Text::new_styled(self.prompt.to_string(), help_style()).as_segment()
    }

    fn drawer(&self) -> Option<DrawerContents> {
        None
    }

    fn result(&self, _dependency_state: &DependencyState) -> String {
        let mut result = String::new();

        for (_, (key, value)) in self.pairs.iter().enumerate() {
            result.push_str(&key.value());

            if !value.value().is_empty() {
                result.push_str(&format!(": {}", value.value()));
            }

            result.push('\n');
        }

        result
    }

    fn add_to(self, form: &mut Form) {
        form.add_step(Box::new(self));
    }
}
