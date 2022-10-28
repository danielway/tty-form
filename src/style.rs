use tty_interface::{Color, Style};

pub(crate) fn help_style() -> Style {
    Style::default().set_foreground(Color::DarkYellow)
}

pub(crate) fn drawer_style() -> Style {
    Style::default().set_foreground(Color::Blue)
}
