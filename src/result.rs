/// A form operation's result containing either a successful value or error.
pub type Result<T> = std::result::Result<T, Error>;

/// A failed form operation's error information.
#[derive(Debug)]
pub enum Error {
    /// A terminal interface error.
    Interface(tty_interface::Error),
    /// A low-level terminal interaction error.
    Terminal(crossterm::ErrorKind),
}

impl From<tty_interface::Error> for Error {
    fn from(err: tty_interface::Error) -> Self {
        Error::Interface(err)
    }
}

impl From<crossterm::ErrorKind> for Error {
    fn from(err: crossterm::ErrorKind) -> Self {
        Error::Terminal(err)
    }
}
