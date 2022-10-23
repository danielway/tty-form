//! # tty-form
//!
//! Provides simple TTY-based user input form capabilities including multi-step forms and complex input types.
//!

mod form;
pub use form::Form;

mod step;
pub use step::{CompoundStep, Step, TextBlockStep};

mod control;
pub use control::{Control, SelectInput, SelectInputOption, StaticText, TextInput};

mod dependency;
pub use dependency::{Action, DependencyId, DependencyState, Evaluation};

mod device;
pub use device::{InputDevice, StdinDevice};

mod result;
pub use result::{Error, Result};

pub mod test;
