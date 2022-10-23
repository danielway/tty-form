use crossterm::event::{Event, KeyCode, KeyModifiers};
use tty_interface::{pos, Interface, Position};

use crate::{dependency::DependencyState, step::InputResult, InputDevice, Result, Step};

/// A TTY-based form with multiple steps and inputs.
///
/// # Examples
/// ```
/// # use tty_interface::{Interface, test::VirtualDevice};
/// # use tty_form::{Error, test::VirtualInputDevice};
/// # let mut device = VirtualDevice::new();
/// # let mut interface = Interface::new(&mut device)?;
/// # let mut stdin = VirtualInputDevice;
/// use tty_form::{Form, Step, CompoundStep, TextBlockStep, Control, TextInput};
///
/// let mut form = Form::new();
///
/// let mut name_step = CompoundStep::new();
/// TextInput::new("Enter a name:", false).add_to_step(&mut name_step);
/// name_step.add_to_form(&mut form);
///
/// TextBlockStep::new("Enter a description of this person:").add_to_form(&mut form);
///
/// let submission = form.execute(&mut interface, &mut stdin)?;
/// # Ok::<(), Error>(())
/// ```
pub struct Form {
    steps: Vec<Box<dyn Step>>,

    /// The currently-focused step.
    active_step: usize,

    /// The furthest step the user has reached so far.
    max_step: usize,

    /// The last render's height.
    last_height: u16,
}

impl Default for Form {
    /// Create a new, default terminal form.
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            active_step: 0,
            max_step: 0,
            last_height: 0,
        }
    }
}

impl Form {
    /// Create a new, default terminal form.
    pub fn new() -> Form {
        Self::default()
    }

    /// Append and return a compound step with multiple component controls.
    pub fn add_step(&mut self, step: Box<dyn Step>) {
        self.steps.push(step);
    }

    /// Execute the provided form and return its WYSIWYG result.
    pub fn execute<D: InputDevice>(
        mut self,
        interface: &mut Interface,
        input_device: &mut D,
    ) -> Result<String> {
        for step in &mut self.steps {
            step.initialize();
        }

        let mut dependency_state = DependencyState::new();

        self.render_form(interface, &dependency_state);
        interface.apply()?;

        loop {
            interface.set_cursor(None);

            if let Event::Key(key_event) = input_device.read()? {
                if (KeyModifiers::CONTROL, KeyCode::Char('c'))
                    == (key_event.modifiers, key_event.code)
                {
                    break;
                }

                if let Some(action) =
                    self.steps[self.active_step].handle_input(key_event, &mut dependency_state)
                {
                    match action {
                        InputResult::AdvanceForm => {
                            if self.advance() {
                                break;
                            }
                        }
                        InputResult::RetreatForm => {
                            if self.retreat() {
                                break;
                            }
                        }
                    }
                }
            }

            self.render_form(interface, &dependency_state);
            interface.apply()?;
        }

        let mut result = String::new();

        for step in self.steps {
            result.push_str(&step.get_result(&dependency_state));
        }

        Ok(result)
    }

    /// Advance the form to its next step. Returns whether we've finished the form.
    fn advance(&mut self) -> bool {
        let is_last_step = self.active_step + 1 == self.steps.len();
        if !is_last_step {
            self.active_step += 1;

            if self.active_step > self.max_step {
                self.max_step = self.active_step;
            }
        }

        is_last_step
    }

    /// Retreat the form to its previous step. Returns whether we're at the first step.
    fn retreat(&mut self) -> bool {
        let is_first_step = self.active_step == 0;
        if !is_first_step {
            self.active_step -= 1;
        }

        is_first_step
    }

    /// Re-render the form's updated state.
    fn render_form(&mut self, interface: &mut Interface, dependency_state: &DependencyState) {
        for line in 0..self.last_height {
            interface.clear_line(line);
        }

        let mut drawer = None;
        let mut line = 1;
        for (step_index, step) in self.steps.iter_mut().enumerate() {
            if step_index > self.max_step {
                break;
            }

            let step_height = step.render(
                pos!(0, line),
                interface,
                step_index == self.active_step,
                dependency_state,
            );
            line += step_height;

            if step_index == self.active_step {
                interface.set(pos!(0, 0), &step.get_help_text());
                drawer = step.get_drawer();
            }
        }

        if let Some(drawer) = drawer {
            for item in drawer {
                interface.set(pos!(0, line), &item);
                line += 1;
            }
        }

        self.last_height = line;
    }
}
