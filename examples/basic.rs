use std::io::stdout;

use tty_form::{
    control::{Control, SelectInput, StaticText, TextInput},
    dependency::{Action, Evaluation},
    device::StdinDevice,
    step::{CompoundStep, KeyValueStep, Step, TextBlockStep, YesNoStep},
    Error, Form, Result,
};
use tty_interface::Interface;

fn main() {
    let result = execute().expect("executes basic example");
    println!("Result:");
    println!("{}", result);
    println!("Done.");
}

fn execute() -> Result<String> {
    let mut form = Form::new();

    let mut commit_summary = CompoundStep::new();
    commit_summary.set_max_line_length(80);

    SelectInput::new(
        "Select the commit type.",
        vec![
            ("feat", "implemented a new feature"),
            ("bug", "fixed existing behavior"),
            ("docs", "added documentation"),
            ("chore", "non-source changes"),
        ],
    )
    .add_to(&mut commit_summary);

    let mut opening_paren = StaticText::new("(");
    let mut closing_paren = StaticText::new(")");

    let mut scope_input = TextInput::new("Enter the commit's scope.", true);

    let empty_scope = scope_input.set_evaluation(Evaluation::IsEmpty);
    opening_paren.set_dependency(empty_scope, Action::Hide);
    closing_paren.set_dependency(empty_scope, Action::Hide);

    let mut breaking_bang = StaticText::new("!");
    let colon = StaticText::new(": ");

    let description = TextInput::new("Enter the commit's description.", true);

    let mut long_description = TextBlockStep::new("Enter a long-form commit description.");
    long_description.set_margins(Some(1), Some(1));
    long_description.set_max_line_length(100);

    let mut breaking_step = YesNoStep::new(
        "Is this commit a breaking change?",
        "Enter a description of the breaking change.",
        "BREAKING CHANGE",
    );

    let trailers = KeyValueStep::new("Enter any key-value trailers, such as tickets.");

    let breaking_change = breaking_step.set_evaluation(Evaluation::Equal("Yes".to_string()));
    breaking_bang.set_dependency(breaking_change, Action::Show);

    opening_paren.add_to(&mut commit_summary);
    scope_input.add_to(&mut commit_summary);
    closing_paren.add_to(&mut commit_summary);
    breaking_bang.add_to(&mut commit_summary);
    colon.add_to(&mut commit_summary);
    description.add_to(&mut commit_summary);
    commit_summary.add_to(&mut form);
    long_description.add_to(&mut form);
    trailers.add_to(&mut form);
    breaking_step.add_to(&mut form);

    let mut stdout = stdout();
    let mut stdin = StdinDevice;

    let mut interface = Interface::new_relative(&mut stdout)?;

    let result = form.execute(&mut interface, &mut stdin);
    interface.exit()?;

    let mut output = String::new();
    match result {
        Ok(value) => output = value,
        Err(Error::Canceled) => println!("Form canceled."),
        Err(err) => eprintln!("Unexpected error occurred: {:?}", err),
    }

    Ok(output)
}
