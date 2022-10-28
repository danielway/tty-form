use tty_interface::{pos, Interface, Position};

use crate::text::Segment;

/// Renders a segment at the specified position, returning the cursor's position after the render.
pub(crate) fn render_segment(
    interface: &mut Interface,
    mut position: Position,
    segment: Segment,
) -> Position {
    for text in segment {
        match text.style() {
            Some(style) => interface.set_styled(position, text.content(), *style),
            None => interface.set(position, text.content()),
        };

        position = pos!(position.x() + text.content().len() as u16, position.y());
    }

    position
}
