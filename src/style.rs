use tty_interface::{Color, Style};

pub(crate) fn help_style() -> Style {
    Style::default().set_foreground(Color::DarkYellow)
}

pub(crate) fn drawer_style() -> Style {
    Style::default().set_foreground(Color::Blue)
}

pub(crate) fn drawer_selected_style() -> Style {
    Style::default().set_foreground(Color::Cyan)
}

pub(crate) fn error_style() -> Style {
    Style::default().set_foreground(Color::Red)
}
