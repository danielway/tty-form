use std::fmt::Formatter;

/// Indicates whether a form operation was successful or failed.
pub type Result<T> = std::result::Result<T, Error>;

/// Failure modes for interface operations.
#[derive(Debug)]
pub enum Error {
    /// The specified step index was out of bounds.
    StepOutOfBounds,
    /// The specified element index was out of bounds.
    ElementOutOfBounds,
    /// An error occurred while updating the interface.
    Interface(tty_interface::Error),
    /// A low-level IO error occurred while performing interface operations.
    IO(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::StepOutOfBounds => write!(f, "Step reference index is out-of-bounds."),
            Error::ElementOutOfBounds => write!(f, "Element reference index is out-of-bounds."),
            Error::Interface(..) => write!(f, "Failure updating interface."),
            Error::IO(..) => write!(f, "Failure interacting with TTY device."),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::StepOutOfBounds => None,
            Error::ElementOutOfBounds => None,
            Error::Interface(ref err) => Some(err),
            Error::IO(ref err) => Some(err),
        }
    }
}

impl From<tty_interface::Error> for Error {
    fn from(err: tty_interface::Error) -> Self {
        Error::Interface(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}
