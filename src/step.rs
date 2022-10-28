use crossterm::event::KeyEvent;
use tty_interface::{Interface, Position};

use crate::{
    dependency::DependencyState,
    text::{DrawerContents, Segment},
    Form,
};

pub mod compound;
pub mod keyvalue;
pub mod textblock;
pub mod yesno;

/// A distinct, vertically-separated phase of the form.
pub trait Step {
    /// Perform any post-configuration initialization actions for this step.
    fn initialize(&mut self, dependency_state: &mut DependencyState, index: usize);

    /// Render this step at the specified position and return the height of the rendered content.
    fn render(
        &self,
        interface: &mut Interface,
        dependency_state: &DependencyState,
        position: Position,
        is_focused: bool,
    ) -> u16;

    /// Handle the specified input event, optionally returning an instruction for the form.
    fn update(
        &mut self,
        dependency_state: &mut DependencyState,
        input: KeyEvent,
    ) -> Option<InputResult>;

    /// Retrieve this step's current help text.
    fn help(&self) -> Segment;

    /// Retrieve this step's current drawer contents, if applicable.
    fn drawer(&self) -> Option<DrawerContents>;

    /// Retrieves this step's final WYSIWYG result.
    fn result(&self, dependency_state: &DependencyState) -> String;

    /// Complete configuration and add this step to the form.
    fn add_to(self, form: &mut Form);
}

/// After processing an input event, an action may be returned to the form from the step.
pub enum InputResult {
    /// Advance the form to the next step.
    AdvanceForm,
    /// Retreat the form to the previous step.
    RetreatForm,
}
