//! # tty-form
//!
//! Provides simple TTY-based user input form capabilities including multi-step forms and complex input types.
//!

mod form;
pub use form::Form;

pub mod control;
pub mod dependency;
pub mod device;
pub mod step;
pub mod style;
pub mod test;
pub mod text;

pub(crate) mod utility;

mod result;
pub use result::{Error, Result};
