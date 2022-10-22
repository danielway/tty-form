use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use tty_interface::{Interface, Position, pos};

use crate::{Result, Step, step::InputResult};

/// A TTY-based form with multiple steps and inputs.
///
/// # Examples
/// ```
/// use tty_form::Form;
///
/// let mut form = Form::new();
///
/// let mut name_step = form.add_compound_step();
/// let mut prompt_text = name_step.add_static_text();
/// prompt_text.set_text("Enter name:");
/// name_step.add_text_input();
///
/// let mut description_step = form.add_description_step();
/// description_step.set_text("Enter information about this person:");
///
/// form.add_text_block_step();
///
/// let submission = form.execute();
/// ```
pub struct Form {
    steps: Vec<Box<dyn Step>>,

    /// The currently-focused step.
    active_step: usize,

    /// The furthest step the user has reached so far.
    max_step: usize,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            active_step: 0,
            max_step: 0,
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
    pub fn execute(mut self, interface: &mut Interface) -> Result<String> {
        for step in &mut self.steps {
            step.initialize();
        }

        self.render_form(interface);
        interface.apply()?;

        loop {
            interface.set_cursor(None);

            if let Event::Key(key_event) = read()? {
                if (KeyModifiers::CONTROL, KeyCode::Char('c')) == (key_event.modifiers, key_event.code) {
                    break;
                }

                if let Some(action) = self.steps[self.active_step].handle_input(key_event) {
                    match action {
                        InputResult::AdvanceForm => {
                            if self.advance() {
                                break;
                            }
                        }
                        InputResult::RetreatForm =>  {
                            if self.retreat() {
                                break;
                            }
                        }
                    }
                }
            }

            self.render_form(interface);
            interface.apply()?;
        }

        Ok(String::new())
    }

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

    fn retreat(&mut self) -> bool {
        let is_first_step = self.active_step == 0;
        if !is_first_step {
            self.active_step -= 1;
        }
        
        is_first_step
    }

    fn render_form(&mut self, interface: &mut Interface) {
        let mut line = 0;
        for (step_index, step) in self.steps.iter_mut().enumerate() {
            if step_index > self.max_step {
                break;
            }

            step.render(pos!(0, line), interface);
            line += 1;
        }
    }
}
