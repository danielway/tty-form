use crate::{step::DescriptionStep, CompoundStep, Step, TextBlockStep};

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
    steps: Vec<Step>,
}

impl Default for Form {
    fn default() -> Self {
        Self { steps: Vec::new() }
    }
}

impl Form {
    /// Create a new, default terminal form.
    pub fn new() -> Form {
        Self::default()
    }

    /// Append and return a compound step with multiple component controls.
    pub fn add_compound_step(&mut self) -> &mut CompoundStep {
        self.steps.push(Step::Compound(CompoundStep::new()));
        match self.steps.last_mut().unwrap() {
            Step::Compound(step) => step,
            _ => panic!(),
        }
    }

    /// Append and return a text block step with a single long-form text entry.
    pub fn add_text_block_step(&mut self) -> &mut TextBlockStep {
        self.steps.push(Step::TextBlock(TextBlockStep::new()));
        match self.steps.last_mut().unwrap() {
            Step::TextBlock(step) => step,
            _ => panic!(),
        }
    }

    /// Append and return a unfocusable, descriptive step with static text only.
    pub fn add_description_step(&mut self) -> &mut DescriptionStep {
        self.steps.push(Step::Description(DescriptionStep::new()));
        match self.steps.last_mut().unwrap() {
            Step::Description(step) => step,
            _ => panic!(),
        }
    }

    /// Execute the provided form and return its WYSIWYG result.
    pub fn execute(self) -> String {
        // TODO
        String::new()
    }
}
