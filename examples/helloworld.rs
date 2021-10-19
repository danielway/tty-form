// use tty_form::schema::TextControl;
use tty_form::control::OptionControl;
use tty_form::form::TTYForm;
use tty_form::schema::Schema;
use tty_form::step::Step;

fn main() {
    // let text_input = TextControl::new();
    let options = vec!["Option 1".to_string(), "Option 2".to_string()];
    let option_input = OptionControl::new(options);

    let mut step = Step::new();
    step.add_control(Box::new(option_input));

    let mut schema = Schema::new();
    schema.add_step(step);

    let form = TTYForm::new();
    form.render(schema);
}
