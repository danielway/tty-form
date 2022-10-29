use tty_interface::{Color, Style};

pub(crate) fn help_style() -> Style {
    Color::DarkYellow.as_style()
}

pub(crate) fn drawer_style() -> Style {
    Color::Blue.as_style()
}

pub(crate) fn drawer_selected_style() -> Style {
    Color::Cyan.as_style()
}

pub(crate) fn error_style() -> Style {
    Color::Red.as_style()
}

pub(crate) fn muted_style() -> Style {
    Color::DarkGrey.as_style()
}
