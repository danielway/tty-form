//! # tty-form
//!
//! Provides simple TTY-based user input form capabilities including multi-step forms and complex input types.
//!

mod form;
pub use form::Form;

mod step;
pub use step::{Step, CompoundStep, TextBlockStep};

mod control;
pub use control::{Control, StaticText, TextInput, SelectInput, SelectInputOption};

mod result;
pub use result::{Error, Result};
