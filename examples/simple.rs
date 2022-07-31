use std::io::stdout;
use termion::raw::IntoRawMode;
use tty_form::element::input::text::Text;
use tty_form::element::literal::Literal;
use tty_form::form::Form;
use tty_form::step::Step;
use tty_form::Result;
use tty_interface::config::Configuration;
use tty_interface::mode::{CursorMode, RenderMode};
use tty_interface::Interface;

fn main() {
    run_form().expect("Execute form");
}

fn run_form() -> Result<()> {
    let mut form = Form::new(vec![Step::new(vec![
        Box::new(Literal::new("Enter text: ".to_string())),
        Box::new(Text::new(true)),
    ])]);

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let config = Configuration::new(CursorMode::Relative, RenderMode::Relative);
    let mut interface = Interface::new_with_configuration(&mut stdout, config)?;

    form.execute(&mut interface)?;

    Ok(())
}
