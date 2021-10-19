use crate::control::Control;
use termion::event::Key;
use tty_interface::line::Line;
use tty_interface::segment::Segment;

pub struct Step {
    pub(crate) controls: Vec<Box<dyn Control>>,
}

impl Step {
    pub fn new() -> Step {
        Step {
            controls: Vec::new(),
        }
    }

    pub fn controls(&self) -> &Vec<Box<dyn Control>> {
        &self.controls
    }

    pub fn add_control(&mut self, control: Box<dyn Control>) {
        self.controls.push(control);
    }

    pub(crate) fn render(&self) -> Line {
        let mut segments: Vec<Segment> = Vec::new();
        for control in &self.controls {
            segments.push(control.render());
        }
        Line::new(segments)
    }

    pub(crate) fn handle_input(&mut self, key: Key) {
        for control in &mut self.controls {
            control.handle_input(key);
        }
    }
}
