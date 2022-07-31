use std::io::stdout;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tty_form::form::Form;
use tty_form::step::Step;
use tty_form::Result;
use tty_interface::config::Configuration;
use tty_interface::mode::{CursorMode, RenderMode};
use tty_interface::Interface;

fn main() {
    run_form().expect("Execute form");
}

fn run_form() -> Result<()> {
    let mut form = Form::new(vec![
        Step::new(vec![
            Box::new(Dynamic::new("D0".to_string())),
            Box::new(Literal::new("E0S1".to_string())),
            Box::new(Literal::new("E1S1\nE1S2\nE1S3".to_string())),
            Box::new(Literal::new("E2S1".to_string())),
            Box::new(Literal::new("E3S1".to_string())),
            Box::new(Literal::new("E4S1\nE4S2".to_string())),
            Box::new(Literal::new("E5S1".to_string())),
            Box::new(Literal::new("E6S1\nE6S2".to_string())),
        ]),
        Step::new(vec![
            Box::new(Literal::new("E7S1\nE7S2".to_string())),
            Box::new(Literal::new("E8S1".to_string())),
            Box::new(Dynamic::new("D1".to_string())),
            Box::new(Literal::new("E9S1".to_string())),
            Box::new(Literal::new("E10S1\nE10S2".to_string())),
        ]),
    ]);

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let config = Configuration::new(CursorMode::Relative, RenderMode::Relative);
    let mut interface = Interface::new_with_configuration(&mut stdout, config)?;

    form.execute(&mut interface)?;

    Ok(())
}

use tty_form::coordinator::Coordinator;
use tty_form::element::literal::Literal;
use tty_form::element::{Element, ElementId};
use tty_form::layout::LayoutAccessor;
use tty_interface::line::LineId;
use tty_interface::position::RelativePosition;
use tty_interface::segment::SegmentId;

pub struct Dynamic {
    id: Option<ElementId>,
    update: usize,
    text: String,

    debug: Option<SegmentId>,
    s1_id: Option<SegmentId>,
    s2_id: Option<SegmentId>,
    l1_id: Option<LineId>,
}

impl Dynamic {
    pub fn new(text: String) -> Self {
        Self {
            debug: None,
            id: None,
            update: 0,
            s1_id: None,
            s2_id: None,
            l1_id: None,
            text,
        }
    }

    fn id(&self) -> &ElementId {
        self.id.as_ref().unwrap()
    }
}

impl Element for Dynamic {
    fn set_id(&mut self, element_id: ElementId) {
        self.id = Some(element_id)
    }

    fn render(&mut self, coordinator: &mut Coordinator) -> Result<()> {
        if self.debug.is_none() {
            let debug = coordinator.add_segment(self.id());
            self.debug = Some(debug.identifier());
        }

        let debug = coordinator.get_segment_mut(self.id(), &self.debug.unwrap());
        debug.set_text(&format!("[status: update={}]", self.update));

        match self.update {
            0 => {
                if self.s1_id.is_none() {
                    let segment = coordinator.add_segment(self.id());
                    self.s1_id = Some(segment.identifier());
                    segment.set_text(&format!("{}S0", &self.text));
                }

                if self.s2_id.is_some() {
                    coordinator.remove_segment(self.id(), &self.s2_id.unwrap());
                    self.s2_id = None;
                }
            }
            1 => {
                if self.s2_id.is_none() {
                    let segment = coordinator.insert_segment(self.id(), 0);
                    self.s2_id = Some(segment.identifier());
                    segment.set_text(&format!("{}S1", &self.text));
                }

                if self.l1_id.is_some() {
                    coordinator.remove_line(self.id(), &self.l1_id.unwrap());
                    self.l1_id = None;
                }
            }
            2 => {
                if self.l1_id.is_none() {
                    let line = coordinator.add_line(self.id());
                    self.l1_id = Some(line.identifier());
                    let segment = line.add_segment();
                    segment.set_text(&format!("{}S2", &self.text));
                }
            }
            _ => {}
        }

        let line_id = coordinator.get_inline_line_id(self.id());
        coordinator.set_cursor(RelativePosition::new(line_id, self.s1_id.unwrap(), 0));

        Ok(())
    }

    fn update_layout(&mut self, _layout_accessor: &mut LayoutAccessor) {}

    fn is_input(&self) -> bool {
        true
    }

    fn captures_enter(&self) -> bool {
        false
    }

    fn update(&mut self, key: Key) -> bool {
        match key {
            Key::Right => self.update += 1,
            Key::Left => {
                if self.update > 0 {
                    self.update -= 1;
                }
            }
            _ => {}
        };

        false
    }
}
