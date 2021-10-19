use tty_interface::line::Line;

use termion::event::Key;

use crate::step::Step;

pub struct Schema {
    pub(crate) steps: Vec<Step>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema { steps: Vec::new() }
    }

    pub fn steps(&self) -> &Vec<Step> {
        &self.steps
    }

    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step)
    }

    pub(crate) fn render(&mut self) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        for step in &self.steps {
            lines.push(step.render());
        }
        lines
    }

    pub(crate) fn handle_input(&mut self, key: Key) {
        for step in &mut self.steps {
            step.handle_input(key);
        }
    }
}
