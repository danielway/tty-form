use crate::schema::Schema;
use tty_interface::interface::TTYInterface;

extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::stdin;

pub struct TTYForm {
    // TODO: reader/writer
}

impl TTYForm {
    pub fn new() -> TTYForm {
        // TODO: reader/writer
        TTYForm {}
    }

    pub fn render(self, mut schema: Schema) /* TODO: data */
    {
        let stdin = stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        let mut tty = TTYInterface::new(&mut stdout);

        for wrapped_key in stdin.keys() {
            let key = wrapped_key.unwrap();

            match key {
                Key::Char(ch) => {
                    if ch == 'q' {
                        break;
                    }
                }
                _ => {}
            }

            schema.handle_input(key);
            let mut lines = schema.render();

            let mut batch = tty.start_update();
            let mut line_index = 0;
            while !lines.is_empty() {
                batch.set_line(line_index, lines.remove(0));
                line_index += 1;
            }
            tty.perform_update(batch).unwrap();
        }

        tty.end().unwrap();
    }
}
