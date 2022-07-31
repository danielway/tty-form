use crate::coordinator::Coordinator;
use crate::element::{Element, ElementId};
use crate::layout::LayoutAccessor;
use crate::Result;
use termion::event::Key;
use tty_interface::line::LineId;
use tty_interface::position::RelativePosition;
use tty_interface::segment::SegmentId;
use tty_text::layout::{LineLayout, RowLayout};

pub struct Text {
    id: Option<ElementId>,
    multi_line: bool,

    text: tty_text::text::Text,

    segment_id: Option<SegmentId>,
    line_ids: Vec<LineId>,
    segment_ids: Vec<SegmentId>,
}

impl Text {
    pub fn new(multi_line: bool) -> Self {
        Self {
            id: None,
            multi_line,
            text: tty_text::text::Text::new(),
            segment_id: None,
            line_ids: Vec::new(),
            segment_ids: Vec::new(),
        }
    }

    fn id(&self) -> &ElementId {
        &self.id.as_ref().unwrap()
    }

    fn render_single(&mut self, coordinator: &mut Coordinator) -> Result<()> {
        let segment = if let Some(segment_id) = self.segment_id {
            coordinator.get_segment_mut(self.id(), &segment_id)
        } else {
            let segment = coordinator.add_segment(self.id());
            self.segment_id = Some(segment.identifier());
            segment
        };

        segment.set_text(&self.text.value());

        let line_id = coordinator.get_inline_line_id(self.id());
        coordinator.set_cursor(RelativePosition::new(
            line_id,
            self.segment_id.unwrap(),
            self.text.cursor().x() as u16,
        ));

        Ok(())
    }

    fn render_multi(&mut self, coordinator: &mut Coordinator) -> Result<()> {
        let mut line_count = 0;
        for (line_index, line_text) in self.text.lines().iter().enumerate() {
            let segment = if line_index >= self.line_ids.len() {
                let line = coordinator.add_line(self.id());
                self.line_ids.push(line.identifier());

                let segment = line.add_segment();
                self.segment_ids.push(segment.identifier());

                segment
            } else {
                let line = coordinator.get_line_mut(self.id(), &self.line_ids[line_index]);
                line.get_segment_mut(&self.segment_ids[line_index])?
            };

            segment.set_text(line_text);
            line_count += 1;
        }

        if line_count < self.line_ids.len() {
            for line_index in line_count..self.line_ids.len() {
                coordinator.remove_line(self.id(), &self.line_ids[line_index]);
                self.line_ids.remove(line_index);
                self.segment_ids.remove(line_index);
            }
        }

        let position = self.text.cursor();
        let line_id = self.line_ids[position.y()];
        let segment_id = self.segment_ids[position.y()];
        coordinator.set_cursor(RelativePosition::new(
            line_id,
            segment_id,
            position.x() as u16,
        ));

        Ok(())
    }
}

impl Element for Text {
    fn set_id(&mut self, element_id: ElementId) {
        self.id = Some(element_id);
    }

    fn render(&mut self, coordinator: &mut Coordinator) -> Result<()> {
        if self.multi_line {
            self.render_multi(coordinator)
        } else {
            self.render_single(coordinator)
        }
    }

    fn update_layout(&mut self, layout_accessor: &mut LayoutAccessor) {
        if self.text.value().is_empty() {
            return;
        }

        let mut line_layouts = Vec::new();

        if self.multi_line {
            for segment_id in &self.segment_ids {
                let segment_layout = layout_accessor.get_segment(*segment_id).unwrap();

                let mut row_layouts = Vec::new();
                for part_layout in segment_layout.parts() {
                    row_layouts.push(RowLayout::new(part_layout.widths().clone()));
                }

                line_layouts.push(LineLayout::new(row_layouts));
            }
        } else {
            let segment_layout = layout_accessor
                .get_segment(self.segment_id.unwrap())
                .unwrap();

            let mut row_layouts = Vec::new();
            for part_layout in segment_layout.parts() {
                row_layouts.push(RowLayout::new(part_layout.widths().clone()));
            }

            line_layouts.push(LineLayout::new(row_layouts));
        }

        self.text.set_layout(line_layouts);
    }

    fn is_input(&self) -> bool {
        true
    }

    fn captures_enter(&self) -> bool {
        self.multi_line
    }

    fn update(&mut self, key: Key) -> bool {
        let text_key = match key {
            Key::Char('\n') => Some(tty_text::key::Key::Enter),
            Key::Char(ch) => Some(tty_text::key::Key::Char(ch)),
            Key::Left => Some(tty_text::key::Key::Left),
            Key::Right => Some(tty_text::key::Key::Right),
            Key::Up => Some(tty_text::key::Key::Up),
            Key::Down => Some(tty_text::key::Key::Down),
            Key::Backspace => Some(tty_text::key::Key::Backspace),
            Key::Delete => Some(tty_text::key::Key::Delete),
            Key::Home => Some(tty_text::key::Key::Home),
            Key::End => Some(tty_text::key::Key::End),
            _ => None,
        };

        if let Some(key) = text_key {
            self.text.update(key);
        }

        false
    }
}
