use crate::rule::Action;
use enum_iterator::{all, Sequence};
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::iter::Iterator;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Priority {
    queue: Vec<ActionOption>,
}

impl Priority {
    pub fn new(order_queue: Vec<ActionOption>) -> Self {
        Self { queue: order_queue }
    }

    pub fn default() -> Self {
        Self {
            queue: all::<ActionOption>().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Iterates over values (as references).
    pub fn iter(&self) -> std::slice::Iter<ActionOption> {
        self.queue.iter()
    }
}

impl Eq for Priority {}

/// Options for an `Action`. The order of action execution is the order of the variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Sequence)]
pub enum ActionOption {
    Say,
    Wild,
    Draw,
    Repeat,
    Reverse,
    Skip,
}

impl ActionOption {
    /// Returns the corrsponding action
    fn action(self, value: String) -> Action {
        match self {
            Self::Say => Action::Say(value),
            Self::Wild => Action::Wild,
            Self::Draw => Action::Draw,
            Self::Repeat => Action::Repeat,
            Self::Reverse => Action::Reverse,
            Self::Skip => Action::Skip,
        }
    }
}

impl From<Action> for ActionOption {
    fn from(action: Action) -> Self {
        match action {
            Action::Say(_) => ActionOption::Say,
            Action::Wild => ActionOption::Wild,
            Action::Draw => ActionOption::Draw,
            Action::Repeat => ActionOption::Repeat,
            Action::Reverse => ActionOption::Reverse,
            Action::Skip => ActionOption::Skip,
        }
    }
}
