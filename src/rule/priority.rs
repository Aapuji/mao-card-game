/* Created by Om Sharma
 * 
 * This program has a "Priority" struct which represents
 * the order of which rules would be applied. As of now,
 * only the default order is used in the game, but for a
 * future expansion, the order could be randomized each game.
 * 
 * ActionOption is an enum which corresponds to Actions,
 * but don't do anything. The main difference is in the
 * "Say" variant, which for `Action::Say`, has an argument
 * (the message), while in `ActionOption::Say` doesn't.
*/

use crate::rule::Action;
use enum_iterator::{all, Sequence};
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::iter::Iterator;

/// A struct representing the order of actions to be executed when rules are applied on a turn.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Priority {
    queue: Vec<ActionOption>,
}

impl Priority {
    /// Creates a new `Priority` instance from the given order.
    pub fn new(order_queue: Vec<ActionOption>) -> Self {
        Self { queue: order_queue }
    }

    /// Creates a default `Priority` object, which is the order of `ActionOption`s.
    pub fn default() -> Self {
        Self {
            queue: all::<ActionOption>().collect(),
        }
    }

    /// Gets the number of action options in the object.
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
    Draw,
    Repeat,
    Reverse,
    Skip,
}

impl ActionOption {
    /// Returns the corrsponding `Action` for the `ActionOption`.
    fn action(self, value: String) -> Action {
        match self {
            Self::Say => Action::Say(value),
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
            Action::Draw => ActionOption::Draw,
            Action::Repeat => ActionOption::Repeat,
            Action::Reverse => ActionOption::Reverse,
            Action::Skip => ActionOption::Skip,
        }
    }
}

#[derive(Debug)]
pub enum ActionValue {
  String(String),
  U32(u32)
}
