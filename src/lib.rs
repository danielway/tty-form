//! # tty-form
//!
//! Provides simple TTY-based user input form capabilities including multi-step forms and complex input types.
//!

mod form;
pub use form::Form;

mod step;
pub use step::{CompoundStep, KeyValueStep, Step, TextBlockStep, YesNoStep};

mod control;
pub use control::{Control, SelectInput, SelectInputOption, StaticText, TextInput};

mod text;
pub(crate) use text::{get_segment_length, set_segment_style, set_segment_subset_style};
pub use text::{DrawerContents, Segment, Text};

mod dependency;
pub use dependency::{Action, DependencyId, DependencyState, Evaluation};

mod device;
pub use device::{InputDevice, StdinDevice};

mod result;
pub use result::{Error, Result};

mod style;
pub(crate) use style::{drawer_style, drawer_selected_style, error_style, help_style};

mod utility;
pub(crate) use utility::render_segment;

pub mod test;
