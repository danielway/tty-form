use tty_form::schema::TextControl;
use tty_form::schema::Step;
use tty_form::schema::Schema;
use tty_form::form::TTYForm;

fn main() {
	let text_input = TextControl::new();

	let mut step = Step::new();
	step.add_control(Box::new(text_input));

	let mut schema = Schema::new();
	schema.add_step(step);

	let form = TTYForm::new();
	form.render(schema);
}