use tty_interface::segment::Segment;

use termion::event::Key;

pub trait Control {
    fn render(&self) -> Segment;
    fn handle_input(&mut self, key: Key);
    fn value(&self) -> &String;
}

pub struct TextControl {
    pub(crate) value: String,
}

impl TextControl {
    pub fn new() -> TextControl {
        TextControl {
            value: String::new(),
        }
    }
}

impl Control for TextControl {
    fn render(&self) -> Segment {
        Segment::new(self.value.to_string())
    }

    fn handle_input(&mut self, key: Key) {
        match key {
            Key::Char(ch) => {
                let value = &mut self.value;
                value.push(ch);
            }
            _ => {}
        }
    }

    fn value(&self) -> &String {
        &self.value
    }
}

pub struct OptionControl {
    pub(crate) options: Vec<String>,
    pub(crate) selected_option: usize,
    pub(crate) value: String,
}

impl OptionControl {
    pub fn new(options: Vec<String>) -> OptionControl {
        OptionControl {
            options,
            selected_option: 0,
            value: String::new(),
        }
    }
}

impl Control for OptionControl {
    fn render(&self) -> Segment {
        Segment::new(self.value.to_string())
    }

    fn handle_input(&mut self, key: Key) {
        match key {
            Key::Up => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                } else {
                    self.selected_option = self.options.len() - 1;
                }
            }
            Key::Down => {
                if self.selected_option < self.options.len() - 1 {
                    self.selected_option += 1;
                } else {
                    self.selected_option = 0;
                }
            }
            _ => {}
        }

        self.value = self.options[self.selected_option].to_string();
    }

    fn value(&self) -> &String {
        &self.value
    }
}
