use tty_form::{Form, Result, CompoundStep, TextBlockStep};
use tty_interface::Interface;

fn main() {
    execute().expect("executes basic example");
}

fn execute() -> Result<()> {
    let mut interface = Interface::for_stdout()?;
    
    let mut step1 = CompoundStep::new();
    step1.add_select_input();
    step1.add_text_input();
    
    let step2 = TextBlockStep::new();

    let mut form = Form::new();
    form.add_step(Box::new(step1));
    form.add_step(Box::new(step2));
    form.execute(&mut interface)?;

    interface.exit()?;

    Ok(())
}