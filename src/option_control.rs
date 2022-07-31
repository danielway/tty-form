use crate::control::Control;
use crate::render::Render;
use termion::event::Key;
use tty_interface::position::Position;
use tty_interface::segment::Segment;

pub struct OptionControl {
    pub help_text: String,
    pub options: Vec<(String, String)>,
    pub selected_option: usize,
}

impl Control for OptionControl {
    fn handle_input(&mut self, key: Key) -> bool {
        match key {
            Key::Down => {
                if self.selected_option + 1 == self.options.len() {
                    self.selected_option = 0;
                } else {
                    self.selected_option += 1;
                }
            }
            Key::Up => {
                if self.selected_option == 0 {
                    self.selected_option = self.options.len() - 1;
                } else {
                    self.selected_option -= 1;
                }
            }
            _ => {}
        };

        false
    }

    fn captures_enter(&self) -> bool {
        false
    }

    fn render(
        &self,
    ) -> (
        Render,
        Option<Position>,
        Option<(Vec<(String, String)>, usize)>,
    ) {
        (
            Render::Inline(vec![Segment::new(
                &self.options[self.selected_option].0.clone(),
            )]),
            None,
            Some((self.options.clone(), self.selected_option)),
        )
    }

    fn get_help_text(&self) -> Option<String> {
        Some(self.help_text.clone())
    }

    fn is_focusable(&self) -> bool {
        true
    }
}
