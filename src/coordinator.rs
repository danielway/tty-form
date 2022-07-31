use crate::element::ElementId;
use crate::form::Form;
use crate::Result;
use std::collections::{BTreeMap, HashMap};
use tty_interface::layout::InterfaceLayout;
use tty_interface::line::{Line, LineId};
use tty_interface::position::{Position, RelativePosition};
use tty_interface::segment::{Segment, SegmentId};
use tty_interface::Interface;

/// Provides controlled access to a terminal interface. Coordinates allocation of lines and segments
/// on the screen to elements.
pub struct Coordinator<'a, 'b> {
    elements: BTreeMap<ElementId, ElementData>,
    inline_lines: HashMap<LineId, Vec<ElementId>>,
    interface: &'a mut Interface<'b>,
}

impl Coordinator<'_, '_> {
    /// Create a new coordinator wrapping the specified interface.
    pub(crate) fn new<'a, 'b>(interface: &'a mut Interface<'b>) -> Coordinator<'a, 'b> {
        Coordinator {
            elements: BTreeMap::new(),
            inline_lines: HashMap::new(),
            interface,
        }
    }

    /// Initializes the specified element's interface data. Invocation order is important.
    pub(crate) fn initialize_elements(&mut self, form: &mut Form) {
        // Make a first pass to initialize element data and initial step inline-lines
        for (step_index, step) in form.steps_mut().iter_mut().enumerate() {
            // Add an inline-line and initialize its element mappings
            let inline_line = self.interface.add_line();
            let line_elements = self
                .inline_lines
                .entry(inline_line.identifier())
                .or_insert(Vec::new());

            // Initialize element data and append them to their initial inline-line
            for (element_index, element) in step.elements_mut().iter_mut().enumerate() {
                let element_id = ElementId::new(step_index, element_index);
                element.set_id(element_id);

                let element_data = ElementData {
                    inline_line_id: inline_line.identifier(),
                    segment_ids: Vec::new(),
                    block_line_ids: Vec::new(),
                };
                self.elements.insert(element_id, element_data);

                line_elements.push(element_id);
            }
        }
    }

    /// Set the cursor's relative position.
    pub fn set_cursor(&mut self, cursor: RelativePosition) {
        self.interface.set_cursor(Position::Relative(cursor));
    }

    /// Hide the cursor.
    pub fn hide_cursor(&mut self) {
        self.interface.hide_cursor();
    }

    /// Applies staged changes for this coordinator's underlying interface.
    pub(crate) fn apply_changes(&mut self) -> Result<InterfaceLayout> {
        let layout = self.interface.apply_changes()?;
        Ok(layout)
    }

    /// Retrieves the specified element's inline segments.
    pub fn segments(&self, element_id: &ElementId) -> Vec<&Segment> {
        let data = self.elements.get(element_id).unwrap();

        let line = self.interface.get_line(&data.inline_line_id).unwrap();
        line.get_segments(&data.segment_ids).unwrap()
    }

    /// Retrieve the specified element's inline segment.
    pub fn get_segment(&self, element_id: &ElementId, segment_id: &SegmentId) -> &Segment {
        let data = self.elements.get(element_id).unwrap();

        let line = self.interface.get_line(&data.inline_line_id).unwrap();
        line.get_segment(segment_id).unwrap()
    }

    /// Retrieve a mutable reference to the specified element's inline segment.
    pub fn get_segment_mut(
        &mut self,
        element_id: &ElementId,
        segment_id: &SegmentId,
    ) -> &mut Segment {
        let data = self.elements.get_mut(element_id).unwrap();

        let line = self.interface.get_line_mut(&data.inline_line_id).unwrap();
        line.get_segment_mut(segment_id).unwrap()
    }

    /// Retrieve the specified element's inline line ID. This ID should be considered dirty
    /// between updates and after any block line updates.
    pub fn get_inline_line_id(&self, element_id: &ElementId) -> LineId {
        self.elements.get(element_id).unwrap().inline_line_id
    }

    /// Add an inline segment to the specified element and return a mutable reference to it.
    pub fn add_segment(&mut self, element_id: &ElementId) -> &mut Segment {
        let data = self.elements.get(element_id).unwrap();

        let line = self.interface.get_line(&data.inline_line_id).unwrap();
        let index = match data.segment_ids.last() {
            Some(last_id) => line.get_segment_index(last_id).unwrap() + 1,
            None => self.get_element_segment_index(element_id),
        };

        let data = self.elements.get_mut(element_id).unwrap();

        let line = self.interface.get_line_mut(&data.inline_line_id).unwrap();
        let segment = line.insert_segment(index).unwrap();
        data.segment_ids.push(segment.identifier());

        segment
    }

    /// Insert an inline segment at the specified index and return a mutable reference to it.
    pub fn insert_segment(&mut self, element_id: &ElementId, index: usize) -> &mut Segment {
        let data = self.elements.get(element_id).unwrap();

        let line = self.interface.get_line(&data.inline_line_id).unwrap();
        let index = match data.segment_ids.first() {
            Some(first_id) => line.get_segment_index(first_id).unwrap() + index,
            None => self.get_element_segment_index(element_id),
        };

        let data = self.elements.get_mut(element_id).unwrap();

        let line = self.interface.get_line_mut(&data.inline_line_id).unwrap();
        let segment = line.insert_segment(index).unwrap();
        data.segment_ids.push(segment.identifier());

        segment
    }

    /// Remove the specified inline segment from the element.
    pub fn remove_segment(&mut self, element_id: &ElementId, segment_id: &SegmentId) {
        let data = self.elements.get_mut(element_id).unwrap();

        let line = self.interface.get_line_mut(&data.inline_line_id).unwrap();
        line.remove_segment(segment_id).unwrap();

        let index = data.segment_ids.iter().position(|id| id == segment_id);
        data.segment_ids.remove(index.unwrap());
    }

    /// Remove the inline segment at the specified index from the element.
    pub fn remove_segment_at(&mut self, element_id: &ElementId, index: usize) {
        let data = self.elements.get(element_id).unwrap();

        let line = self.interface.get_line(&data.inline_line_id).unwrap();
        let index = match data.segment_ids.first() {
            Some(first_id) => line.get_segment_index(first_id).unwrap() + index,
            None => self.get_element_segment_index(element_id),
        };

        let data = self.elements.get_mut(element_id).unwrap();

        let line = self.interface.get_line_mut(&data.inline_line_id).unwrap();
        line.remove_segment_at(index).unwrap();

        data.segment_ids.remove(index);
    }

    /// Retrieves the specified element's block lines.
    pub fn lines(&self, element_id: &ElementId) -> Vec<&Line> {
        let data = self.elements.get(element_id).unwrap();
        self.interface.get_lines(&data.block_line_ids).unwrap()
    }

    /// Retrieve a specific block line for the specified element.
    pub fn get_line(&self, _element_id: &ElementId, line_id: &LineId) -> &Line {
        self.interface.get_line(line_id).unwrap()
    }

    /// Retrieve a mutable reference to a specified block line for the specified element.
    pub fn get_line_mut(&mut self, _element_id: &ElementId, line_id: &LineId) -> &mut Line {
        self.interface.get_line_mut(line_id).unwrap()
    }

    /// Append a block line to the specified element and return a mutable reference to it.
    pub fn add_line(&mut self, element_id: &ElementId) -> &mut Line {
        let data = self.elements.get(element_id).unwrap();

        if data.block_line_ids.is_empty() {
            self.try_inline_split(element_id);
        }

        let data = self.elements.get(element_id).unwrap();

        let index = match data.block_line_ids.last() {
            Some(last_id) => self.interface.get_line_index(last_id).unwrap() + 1,
            None => self.get_element_line_index(element_id),
        };

        let data = self.elements.get_mut(element_id).unwrap();

        let line = self.interface.insert_line(index).unwrap();
        data.block_line_ids.push(line.identifier());

        line
    }

    /// Insert a block line at the specified index and return a mutable reference to it.
    pub fn insert_line(&mut self, element_id: &ElementId, index: usize) -> &mut Line {
        let data = self.elements.get(element_id).unwrap();

        if data.block_line_ids.is_empty() {
            self.try_inline_split(element_id);
        }

        let data = self.elements.get(element_id).unwrap();

        let line_index = match data.block_line_ids.first() {
            Some(first_id) => self.interface.get_line_index(first_id).unwrap() + index,
            None => self.get_element_line_index(element_id),
        };

        let data = self.elements.get_mut(element_id).unwrap();

        let line = self.interface.insert_line(line_index).unwrap();
        data.block_line_ids.insert(index, line.identifier());

        line
    }

    /// Remove the specified block line from the element.
    pub fn remove_line(&mut self, element_id: &ElementId, line_id: &LineId) {
        let data = self.elements.get(element_id).unwrap();

        if data.block_line_ids.len() == 1 {
            self.try_inline_join(element_id);
        }

        self.interface.remove_line(line_id).unwrap();

        let data = self.elements.get_mut(element_id).unwrap();

        let index = data.block_line_ids.iter().position(|id| id == line_id);
        data.block_line_ids.remove(index.unwrap());
    }

    /// Remove a block line from the specified index.
    pub fn remove_line_at(&mut self, element_id: &ElementId, index: usize) {
        let data = self.elements.get(element_id).unwrap();

        if data.block_line_ids.len() == 1 {
            self.try_inline_join(element_id);
        }

        let data = self.elements.get(element_id).unwrap();

        let line_index = match data.block_line_ids.first() {
            Some(first_id) => self.interface.get_line_index(first_id).unwrap() + index,
            None => self.get_element_line_index(element_id),
        };

        self.interface.remove_line_at(line_index).unwrap();

        let data = self.elements.get_mut(element_id).unwrap();
        data.block_line_ids.remove(index);
    }

    /// Attempt to perform an inline-line split in response to the specified element's block lines
    /// expanding. May be a no-op if no subsequent elements' segments occupy the same inline-line.
    fn try_inline_split(&mut self, element_id: &ElementId) {
        let data = self.elements.get(element_id).unwrap();

        // Determine the next element's segment's index
        let element_segment_index = self.get_element_segment_index(element_id);
        let next_segment_index = element_segment_index + data.segment_ids.len();

        let line_id = data.inline_line_id;
        let elements = self.inline_lines.get(&line_id).unwrap();
        let subsequent_elements = get_subsequent_elements(elements, element_id);

        // If there are elements past this element, we have to split the inline-line
        if !subsequent_elements.is_empty() {
            // Perform the line split
            let new_line_id = self.split_line(&line_id, next_segment_index);

            // Remove subsequent elements from the old inline line
            let elements = self.inline_lines.get_mut(&line_id).unwrap();
            let element_index = elements.iter().position(|id| id == element_id).unwrap();
            for _ in (element_index + 1)..elements.len() {
                elements.remove(element_index + 1);
            }

            // Add subsequent elements to the new inline line
            self.inline_lines.insert(new_line_id, subsequent_elements);
            let subsequent_elements = self.inline_lines.get(&new_line_id).unwrap();

            // Update subsequent elements' data for the new inline line ID
            for subsequent_element_id in subsequent_elements {
                let sub_elem_data = self.elements.get_mut(&subsequent_element_id).unwrap();
                sub_elem_data.inline_line_id = new_line_id;
            }
        }
    }

    /// Splits the specified line by creating a new line after it and moving all segments >= the
    /// specified index to the new line.
    fn split_line(&mut self, line_id: &LineId, split_index: usize) -> LineId {
        let line_index = self.interface.get_line_index(line_id).unwrap();
        let new_line_id = self
            .interface
            .insert_line(line_index + 1)
            .unwrap()
            .identifier();

        let line = self.interface.get_line(line_id).unwrap();
        let segment_ids: Vec<SegmentId> = line
            .segment_ids()
            .iter()
            .skip(split_index)
            .map(|id| *id)
            .collect();

        for segment_id in segment_ids {
            self.interface
                .move_segment(&segment_id, line_id, &new_line_id)
                .unwrap();
        }

        new_line_id
    }

    /// Attempt to perform an inline-line join in response to the specified element's block lines
    /// being removed. May be a no-op if no subsequent inline-line exists.
    fn try_inline_join(&mut self, element_id: &ElementId) {
        let data = self.elements.get(element_id).unwrap();

        let inline_line_before_id = data.inline_line_id;
        let inline_line_index = self
            .interface
            .get_line_index(&inline_line_before_id)
            .unwrap();

        // If there aren't any lines past this one, there's no other inline-line to join
        if self.interface.line_ids().len() <= inline_line_index + 2 {
            return;
        }

        // Identify the line following the element's block and determine if it's an inline-line
        let inline_line_after_id = *self
            .interface
            .line_ids()
            .get(inline_line_index + 2)
            .unwrap();
        if self.inline_lines.contains_key(&inline_line_after_id) {
            // Perform the line join
            self.join_inline_lines(&inline_line_before_id, &inline_line_after_id);

            // Remove the old inline-line and take its elements
            let mut inline_line_after = self.inline_lines.remove(&inline_line_after_id).unwrap();

            // Update its elements' data to point to the new inline-line
            for moved_element in &inline_line_after {
                let moved_element_data = self.elements.get_mut(moved_element).unwrap();
                moved_element_data.inline_line_id = inline_line_before_id;
            }

            // Append the moved elements to the inline-line
            let inline_line_before = self.inline_lines.get_mut(&inline_line_before_id).unwrap();
            inline_line_before.append(&mut inline_line_after);
        }
    }

    /// Merges the specified lines by appending the latter's to the former.
    fn join_inline_lines(&mut self, first_line_id: &LineId, second_line_id: &LineId) {
        let second_line = self.interface.get_line(second_line_id).unwrap();
        let segment_ids: Vec<SegmentId> = second_line.segment_ids().to_vec();

        for segment_id in segment_ids {
            self.interface
                .move_segment(&segment_id, second_line_id, first_line_id)
                .unwrap();
        }

        self.interface.remove_line(second_line_id).unwrap();
    }

    /// Determine the index of the first segment in the inline-line for the specified element.
    fn get_element_segment_index(&self, element_id: &ElementId) -> usize {
        let data = self.elements.get(element_id).unwrap();
        let elements = self.inline_lines.get(&data.inline_line_id).unwrap();

        let mut segment_count = 0;
        for element in elements.iter().take_while(|id| id != &element_id) {
            let preceding_element_data = self.elements.get(element).unwrap();
            segment_count += preceding_element_data.segment_ids.len();
        }

        segment_count
    }

    /// Determine the index of the first line for the specified element.
    fn get_element_line_index(&self, element_id: &ElementId) -> usize {
        let mut last_line_id = None;
        let mut line_count = 0;

        let element_index = self
            .elements
            .keys()
            .position(|id| id == element_id)
            .unwrap();

        for element_data in self.elements.values().take(element_index + 1) {
            line_count += element_data.block_line_ids.len();

            if last_line_id != Some(element_data.inline_line_id) {
                line_count += 1;
            }

            last_line_id = Some(element_data.inline_line_id);
        }

        line_count
    }
}

struct ElementData {
    inline_line_id: LineId,
    segment_ids: Vec<SegmentId>,
    block_line_ids: Vec<LineId>,
}

/// Given a collection of element IDs and a target ID, returns IDs after the target.
fn get_subsequent_elements(element_ids: &Vec<ElementId>, element_id: &ElementId) -> Vec<ElementId> {
    element_ids
        .iter()
        .skip_while(|id| id != &element_id)
        .skip(1)
        .map(|id| *id)
        .collect()
}
