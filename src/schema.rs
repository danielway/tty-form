use tty_interface::line::Line;
use tty_interface::segment::Segment;
use termion::event::Key;

pub struct Schema {
	pub(crate) steps: Vec<Step>,
}

impl Schema {
	pub fn new() -> Schema {
		Schema {
			steps: Vec::new(),
		}
	}

	pub fn steps(&self) -> &Vec<Step> {
		&self.steps
	}

	pub fn add_step(&mut self, step: Step) {
		self.steps.push(step)
	}

	pub(crate) fn render(&mut self) -> Vec<Line> {
		let mut lines: Vec<Line> = Vec::new();
		for step in &self.steps {
			lines.push(step.render());
		}
		lines
	}

	pub(crate) fn handle_input(&mut self, key: Key) {
		for step in &mut self.steps {
			step.handle_input(key);
		}
	}
}

pub struct Step {
	pub(crate) controls: Vec<Box<dyn Control>>,
}

impl Step {
	pub fn new() -> Step {
		Step {
			controls: Vec::new(),
		}
	}

	pub fn controls(&self) -> &Vec<Box<dyn Control>> {
		&self.controls
	}

	pub fn add_control(&mut self, control: Box<dyn Control>) {
		self.controls.push(control);
	}

	pub(crate) fn render(&self) -> Line {
		let mut segments: Vec<Segment> = Vec::new();
		for control in &self.controls {
			segments.push(control.render());
		}
		Line::new(segments)
	}

	pub(crate) fn handle_input(&mut self, key: Key) {
		for control in &mut self.controls {
			control.handle_input(key);
		}
	}
}

pub trait Control {
	fn render(&self) -> Segment;
	fn handle_input(&mut self, key: Key);
}

pub struct TextControl {
	data: TextControlData,
}

impl TextControl {
	pub fn new() -> TextControl {
		TextControl {
			data: TextControlData::new(),
		}
	}
}

impl Control for TextControl {
	fn render(&self) -> Segment {
		Segment::new(self.data.value().to_string())
	}

	fn handle_input(&mut self, key: Key) {
		match key {
			Key::Char(ch) => {
				let value = &mut self.data.value;
				value.push(ch);
			},
			_ => {},
		}
	}
}

pub struct TextControlData {
	pub(crate) value: String,
}

impl TextControlData {
	pub(crate) fn new() -> TextControlData {
		TextControlData {
			value: String::new(),
		}
	}

	pub fn value(&self) -> &str {
		&self.value
	}
}
