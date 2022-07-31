use crate::coordinator::Coordinator;
use crate::element::{Element, ElementId};
use crate::layout::LayoutAccessor;
use crate::Result;
use termion::event::Key;

pub struct Literal {
    id: Option<ElementId>,
    text: String,
    rendered: bool,
}

impl Literal {
    pub fn new(text: String) -> Self {
        Self {
            text,
            id: None,
            rendered: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Element for Literal {
    fn set_id(&mut self, element_id: ElementId) {
        self.id = Some(element_id);
    }

    fn render(&mut self, coordinator: &mut Coordinator) -> Result<()> {
        if !self.rendered {
            for (index, segment_text) in self.text.lines().enumerate() {
                let segment = if index == 0 {
                    coordinator.add_segment(&self.id.unwrap())
                } else {
                    let line = coordinator.add_line(&self.id.unwrap());
                    line.add_segment()
                };

                segment.set_text(segment_text);
            }

            self.rendered = true;
        }

        Ok(())
    }

    fn update_layout(&mut self, _layout_accessor: &mut LayoutAccessor) {}

    fn is_input(&self) -> bool {
        false
    }

    fn captures_enter(&self) -> bool {
        false
    }

    fn update(&mut self, _key: Key) -> bool {
        false
    }
}
