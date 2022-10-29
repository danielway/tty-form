use crossterm::event::KeyEvent;

use crate::{
    dependency::{Action, DependencyId, Evaluation},
    step::CompoundStep,
    text::{DrawerContents, Segment},
};

mod selectinput;
pub use selectinput::*;

mod statictext;
pub use statictext::*;

mod textinput;
pub use textinput::*;

/// An element of a [CompoundStep] which may be a focusable input.
pub trait Control {
    /// Whether this control is a focusable input.
    fn focusable(&self) -> bool;

    /// Updates the control's state from the given input event.
    fn update(&mut self, input: KeyEvent);

    /// This control's descriptive help text, if available.
    fn help(&self) -> Option<Segment>;

    /// This control's rendered contents and an optional offset for the cursor.
    fn text(&self) -> (Segment, Option<u16>);

    /// This control's drawer contents, if available.
    fn drawer(&self) -> Option<DrawerContents>;

    /// This control's dependency evaluation which other controls may react to.
    fn evaluation(&self) -> Option<(DependencyId, Evaluation)>;

    /// This control's dependency which it may react to.
    fn dependency(&self) -> Option<(DependencyId, Action)>;

    /// Perform an evaluation against this control's current state.
    fn evaluate(&self, evaluation: &Evaluation) -> bool;

    /// Finish configuration and add this control to the specified form step.
    fn add_to(self, step: &mut CompoundStep);
}
