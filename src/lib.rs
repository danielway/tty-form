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

mod text;
pub(crate) use text::set_segment_style;
pub use text::{DrawerContents, Segment, Text};

mod dependency;
pub use dependency::{Action, DependencyId, DependencyState, Evaluation};

mod device;
pub use device::{InputDevice, StdinDevice};

mod result;
pub use result::{Error, Result};

mod utility;
pub(crate) use utility::render_segment;

pub mod test;
