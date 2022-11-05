use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

/// A unique identifier.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct DependencyId(usize);

/// The greatest dependency identifier provisioned thus far.
static ID_VALUE: AtomicUsize = AtomicUsize::new(0);

impl DependencyId {
    /// Create a new, unique dependency identifier.
    pub(crate) fn new() -> Self {
        Self(ID_VALUE.fetch_add(1, Ordering::Relaxed))
    }
}

/// An evaluation to apply to the source of a dependency.
#[derive(Clone)]
pub enum Evaluation {
    /// Evaluates true if the source is empty.
    IsEmpty,
    /// Evaluates true if the source's value matches the evaluation parameter.
    Equal(String),
    /// Evaluates true if the source's value is different from the evaluation parameter.
    NotEqual(String),
}

/// An action to apply to the target if the source evaluates true.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Action {
    /// If the evaluation is true for the source, the target is hidden, otherwise it is shown.
    Hide,
    /// If the evaluation is false for the source, the target is shown, otherwise it is hidden.
    Show,
}

pub struct DependencyState {
    /// The latest evaluation value for each dependency.
    evaluation_states: HashMap<DependencyId, bool>,
    /// Maps a dependency to its source (step, control) indices.
    evaluation_sources: HashMap<DependencyId, (usize, usize)>,
}

impl DependencyState {
    pub(crate) fn new() -> Self {
        Self {
            evaluation_states: HashMap::new(),
            evaluation_sources: HashMap::new(),
        }
    }

    pub(crate) fn register_evaluation(&mut self, id: &DependencyId, step: usize, control: usize) {
        self.evaluation_sources.insert(*id, (step, control));
    }

    pub(crate) fn get_source(&self, id: &DependencyId) -> (usize, usize) {
        *self.evaluation_sources.get(id).unwrap()
    }

    pub(crate) fn update_evaluation(&mut self, id: &DependencyId, value: bool) {
        self.evaluation_states.insert(*id, value);
    }

    pub(crate) fn get_evaluation(&self, id: &DependencyId) -> bool {
        *self.evaluation_states.get(id).unwrap_or(&false)
    }
}
