use crate::element::Element;

pub struct Step {
    elements: Vec<Box<dyn Element>>,
}

impl Step {
    pub fn new(elements: Vec<Box<dyn Element>>) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &Vec<Box<dyn Element>> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut Vec<Box<dyn Element>> {
        &mut self.elements
    }

    pub fn get_element(&self, element_index: usize) -> &dyn Element {
        self.elements[element_index].as_ref()
    }

    pub fn get_element_mut(&mut self, element_index: usize) -> &mut dyn Element {
        self.elements[element_index].as_mut()
    }
}
