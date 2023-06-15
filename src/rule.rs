/* Created by Om Sharma
 *
 * This program has a "Rule" struct, which represents a rule in the
 * game. It has an event on which it activates, and an action which
 * happens when the rule is triggered.
 */

use crate::card::{Card, Suit, Value};
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

pub mod priority;
pub mod rule_map;

/// A struct representing an in-game rule
///
/// The structure is: `On EVENT do ACTION`.
#[derive(Debug, Clone, Hash)]
pub struct Rule {
    event: Event,
    action: Action,
}

/// A enum of the possible events that can trigger a rule action or requirement.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Event {
    CardPlayed(Card),
    ValuePlayed(Value),
    SuitPlayed(Suit),
}

/// An enum of the actions that can happen when a rule is triggered.
///
/// `Action::Say` is special, because it is a _requirement_, rather than an _action_. So nothing happens when a `Say` action occurs. Rather, it adds a requirement that the player say something.
///
/// The order of action execution is the order of the variants.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Action {
    Say(String), // Perhaps have a macro that inserts the string into println!, so they can put {card}, {suit}, and {value} in the string to interpolate?. Also, this variant is the only variant which doesn't actually do something, but checks if the player does it correctly.
    Draw,        // Number of cards to draw
    Repeat,
    Reverse,
    Skip,
}

impl Rule {
    /// Creates a new `Rule` from the `event` and `action`.
    pub fn new(event: Event, action: Action) -> Self {
        Self { event, action }
    }

    /// Returns the `Event` which the rule will trigger on.
    pub fn event(&self) -> Event {
        self.event.clone()
    }

    /// Returns the `Action` that will happen once the rule is triggered.
    pub fn action(&self) -> Action {
        self.action.clone()
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event && self.action == other.action
    }
}

impl Eq for Rule {}

impl Event {
    /// Checks if the arg matches the corresponding value in `card`.
    ///
    /// Example: `Event::ValuePlayed(Value::Ace)` and `Card::new(Value::Ace, Suit::Spades)` would match
    pub fn arg_matches(&self, card: Card) -> bool {
        match self {
            Self::CardPlayed(arg) => arg == &card,
            Self::ValuePlayed(arg) => arg == &card.value(),
            Self::SuitPlayed(arg) => arg == &card.suit(),
        }
    }
}
