/// An input device to use for controlling a form.
pub trait InputDevice {
    /// Blocks until an input event is received.
    fn read(&mut self) -> crossterm::Result<crossterm::event::Event>;
}

/// The standard input device.
pub struct StdinDevice;

impl InputDevice for StdinDevice {
    fn read(&mut self) -> crossterm::Result<crossterm::event::Event> {
        crossterm::event::read()
    }
}
