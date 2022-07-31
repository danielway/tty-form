use crate::coordinator::Coordinator;
use crate::element::Element;
use crate::layout::LayoutAccessor;
use crate::step::Step;
use crate::Result;
use std::cmp::max;
use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;
use tty_interface::Interface;

pub struct Form {
    steps: Vec<Step>,

    active_step_index: usize,
    max_step_index: usize,

    active_element_index: usize,
}

impl Form {
    pub fn new(steps: Vec<Step>) -> Self {
        Self {
            steps,
            active_step_index: 0,
            max_step_index: 0,
            active_element_index: 0,
        }
    }

    pub fn steps(&self) -> &Vec<Step> {
        &self.steps
    }

    pub(crate) fn steps_mut(&mut self) -> &mut Vec<Step> {
        &mut self.steps
    }

    pub fn get_step(&self, step_index: usize) -> &Step {
        &self.steps[step_index]
    }

    pub fn get_step_mut(&mut self, step_index: usize) -> &mut Step {
        &mut self.steps[step_index]
    }

    pub fn get_element(&self, step_index: usize, element_index: usize) -> &dyn Element {
        let step = &self.steps[step_index];
        step.get_element(element_index)
    }

    pub(crate) fn get_element_mut(
        &mut self,
        step_index: usize,
        element_index: usize,
    ) -> &mut dyn Element {
        let step = &mut self.steps[step_index];
        step.get_element_mut(element_index)
    }

    pub fn execute(&mut self, interface: &mut Interface) -> Result<()> {
        let mut coordinator = Coordinator::new(interface);
        coordinator.initialize_elements(self);

        if !self.active_element().is_input() {
            self.move_focus_forward();
        }

        self.render(&mut coordinator, true)?;

        let mut input = stdin().keys();
        loop {
            let event = input.next();
            if let Some(key) = event {
                let key = key?;

                match key {
                    Key::Ctrl('c') => {
                        // Quit the form early
                        break;
                    }
                    Key::Char('\n') => {
                        let should_advance;
                        if self.active_element().captures_enter() {
                            should_advance = self.active_element_mut().update(key);
                        } else {
                            should_advance = true;
                        }

                        if should_advance {
                            let reached_end = self.move_focus_forward();
                            if reached_end {
                                break;
                            }
                        }
                    }
                    Key::Esc => {
                        let reached_beginning = self.move_focus_backward();
                        if reached_beginning {
                            break;
                        }
                    }
                    _ => {
                        let should_advance = self.active_element_mut().update(key);
                        if should_advance {
                            let reached_end = self.move_focus_forward();
                            if reached_end {
                                break;
                            }
                        }
                    }
                }
            }

            self.render(&mut coordinator, false)?;
        }

        interface.advance_to_end().unwrap();

        Ok(())
    }

    /// Advance focus to the next element. Returns whether we've reached the end of the form.
    fn move_focus_forward(&mut self) -> bool {
        loop {
            // Check if we've reached the end of this step
            if self.active_element_index + 1 == self.active_step().elements().len() {
                // Check if we've reached the end of the form
                if self.active_step_index + 1 == self.steps().len() {
                    return true;
                }

                self.active_step_index += 1;
                self.active_element_index = 0;
            } else {
                // If we're not at the end of the step, simply advance controls
                self.active_element_index += 1;
            }

            // Stop advancing if we've reached another focusable control
            if self.active_element().is_input() {
                break;
            }
        }

        false
    }

    /// Retreat focus to the previous control or step.
    fn move_focus_backward(&mut self) -> bool {
        loop {
            // Check if we've reached the start of a step
            if self.active_element_index == 0 {
                // Check if we've reached the start of the form
                if self.active_step_index == 0 {
                    return true;
                }

                self.active_step_index -= 1;
                self.active_element_index = self.active_step().elements().len() - 1;
            } else {
                // If we're not at the start of the step, simply retreat controls
                self.active_element_index -= 1;
            }

            // Stop retreating if we've reached another focusable control
            if self.active_element().is_input() {
                break;
            }
        }

        false
    }

    fn render(&mut self, coordinator: &mut Coordinator, first_render: bool) -> Result<()> {
        let (min_step_rendered, max_step_rendered) =
            if first_render || self.max_step_index < self.active_step_index {
                (0, self.active_step_index)
            } else {
                (self.max_step_index, self.active_step_index)
            };

        coordinator.hide_cursor();

        if first_render || max_step_rendered > min_step_rendered {
            for step_index in min_step_rendered..=max_step_rendered {
                let step = self.get_step_mut(step_index);
                for element in step.elements_mut() {
                    element.render(coordinator)?;
                }
            }
        } else {
            self.active_element_mut().render(coordinator)?;
        }

        self.max_step_index = max(self.active_step_index, self.max_step_index);

        let layout = coordinator.apply_changes()?;
        let mut accessor = LayoutAccessor::new(layout);

        if first_render || max_step_rendered > min_step_rendered {
            for step_index in min_step_rendered..=max_step_rendered {
                let step = self.get_step_mut(step_index);
                for element in step.elements_mut() {
                    element.update_layout(&mut accessor);
                }
            }
        } else {
            self.active_element_mut().update_layout(&mut accessor);
        }

        Ok(())
    }

    fn active_step(&self) -> &Step {
        self.get_step(self.active_step_index)
    }

    fn active_element(&self) -> &dyn Element {
        let step_index = self.active_step_index;
        let element_index = self.active_element_index;
        self.get_element(step_index, element_index)
    }

    fn active_element_mut(&mut self) -> &mut dyn Element {
        let step_index = self.active_step_index;
        let element_index = self.active_element_index;
        self.get_element_mut(step_index, element_index)
    }
}
