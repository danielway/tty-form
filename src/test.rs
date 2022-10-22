//! A virtual testing device based on the vte/vt100 parser used in functional and documentation tests.

use crate::InputDevice;

pub struct VirtualInputDevice;

impl InputDevice for VirtualInputDevice {
    fn read(&mut self) -> crossterm::Result<crossterm::event::Event> {
        Ok(crossterm::event::Event::Key(
            crossterm::event::KeyEvent::new(
                crossterm::event::KeyCode::Enter,
                crossterm::event::KeyModifiers::NONE,
            ),
        ))
    }
}
