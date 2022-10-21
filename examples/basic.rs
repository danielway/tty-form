use std::io::stdout;

use tty_form::{Form, Result, CompoundStep, TextBlockStep, SelectInput, SelectInputOption, Control, StaticText, TextInput, Step};
use tty_interface::Interface;

fn main() {
    execute().expect("executes basic example");
}

fn execute() -> Result<()> {
    let mut stdout = stdout();
    let mut interface = Interface::new(&mut stdout)?;
    
    let mut form = Form::new();

    let mut commit_summary = CompoundStep::new();
    
    SelectInput::new("Select the commit type.", vec![
        ("feat", "implemented a new feature"),
        ("bug", "fixed existing behavior"),
        ("docs", "added documentation"),
        ("chore", "non-source changes"),
    ]).add_to_step(&mut commit_summary);

    StaticText::new("(").add_to_step(&mut commit_summary);
    TextInput::new("Enter the commit's scope.").add_to_step(&mut commit_summary);
    StaticText::new("):").add_to_step(&mut commit_summary);
    TextInput::new("Enter the commit's description.").add_to_step(&mut commit_summary);

    commit_summary.add_to_form(&mut form);

    TextBlockStep::new("Enter a long-form commit description.").add_to_form(&mut form);

    form.execute(&mut interface)?;

    interface.exit()?;

    Ok(())
}