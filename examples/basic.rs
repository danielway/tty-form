use std::io::stdout;

use tty_form::{
    Action, CompoundStep, Control, Evaluation, Form, Result, SelectInput, StaticText, StdinDevice,
    Step, TextBlockStep, TextInput,
};
use tty_interface::Interface;

fn main() {
    execute().expect("executes basic example");
}

fn execute() -> Result<()> {
    let mut form = Form::new();

    let mut commit_summary = CompoundStep::new();

    SelectInput::new(
        "Select the commit type.",
        vec![
            ("feat", "implemented a new feature"),
            ("bug", "fixed existing behavior"),
            ("docs", "added documentation"),
            ("chore", "non-source changes"),
        ],
    )
    .add_to_step(&mut commit_summary);

    let mut opening_paren = StaticText::new("(");
    let mut scope_input = TextInput::new("Enter the commit's scope.", true);

    let empty_scope = scope_input.create_dependency_source(Evaluation::IsEmpty);
    opening_paren.add_dependency(empty_scope, Action::Hide);

    let mut closing_paren = StaticText::new(")");
    closing_paren.add_dependency(empty_scope, Action::Hide);

    opening_paren.add_to_step(&mut commit_summary);
    scope_input.add_to_step(&mut commit_summary);
    closing_paren.add_to_step(&mut commit_summary);

    StaticText::new(": ").add_to_step(&mut commit_summary);

    TextInput::new("Enter the commit's description.", true).add_to_step(&mut commit_summary);

    commit_summary.add_to_form(&mut form);

    TextBlockStep::new("Enter a long-form commit description.").add_to_form(&mut form);

    let mut stdout = stdout();
    let mut stdin = StdinDevice;

    let mut interface = Interface::new(&mut stdout)?;
    form.execute(&mut interface, &mut stdin)?;

    interface.exit()?;

    Ok(())
}
