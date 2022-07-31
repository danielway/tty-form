pub mod input;
pub mod literal;

use crate::coordinator::Coordinator;
use crate::layout::LayoutAccessor;
use crate::Result;
use termion::event::Key;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ElementId {
    step_index: usize,
    element_index: usize,
}

impl ElementId {
    pub(crate) fn new(step_index: usize, element_index: usize) -> Self {
        Self {
            step_index,
            element_index,
        }
    }
}

/// An element in the form which may be static content or an input.
pub trait Element {
    fn set_id(&mut self, element_id: ElementId);

    /// Renders this element to the screen and returns the relative cursor position.
    fn render(&mut self, coordinator: &mut Coordinator) -> Result<()>;

    fn update_layout(&mut self, layout_accessor: &mut LayoutAccessor);

    /// Whether this element accepts user input and thus should be focusable.
    fn is_input(&self) -> bool;

    /// Whether this control should capture the `Enter` key, rather than advancing the form.
    fn captures_enter(&self) -> bool;

    /// Update this element's state with the given user input. Return whether to advance the form.
    fn update(&mut self, key: Key) -> bool;
}
