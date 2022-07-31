use tty_interface::layout::{InterfaceLayout, SegmentLayout};
use tty_interface::segment::SegmentId;

#[derive(Debug)]
pub struct LayoutAccessor {
    layout: InterfaceLayout,
}

impl LayoutAccessor {
    pub(crate) fn new(layout: InterfaceLayout) -> LayoutAccessor {
        LayoutAccessor { layout }
    }

    pub(crate) fn get_segment(&self, segment_id: SegmentId) -> Option<&SegmentLayout> {
        for line_layout in self.layout.lines() {
            for segment_layout in line_layout.segments() {
                if segment_layout.segment_id() == Some(segment_id) {
                    return Some(segment_layout);
                }
            }
        }

        None
    }
}
