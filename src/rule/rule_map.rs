/* Created by Om Sharma
 * 
 * The "RuleMap" struct maps ActionOptions to a vector of
 * Rules. These rules are all the rules that apply the
 * action corresponding to the ActionOption. It will be
 * able to create a default RuleMap, push a rule to the
 * end of a vector, get the corresponding vector for an
 * Action.
 */

use crate::rule::priority::ActionOption;
use crate::rule::Rule;
use enum_iterator::all;
use std::collections::HashMap;

/// The key type for a `RuleMap`.
type Key = ActionOption;

/// A struct mapping an `ActionOption` to a collection of rules that have that action.
#[derive(Debug)]
pub struct RuleMap {
    map: HashMap<Key, Vec<Rule>>,
}

impl RuleMap {
    /// Creates a new `RuleMap` from the given `map`.
    pub fn new(map: HashMap<Key, Vec<Rule>>) -> Self {
        Self { map }
    }

    /// Creates a new `RuleMap` from the `ActionOptions`.
    pub fn default() -> Self {
        let mut map = HashMap::new();

        for option in all::<Key>() {
            map.insert(option, vec![]);
        }

        Self { map }
    }

    /// Pushes `rule` to end of the vector at key `option`, or returns an error.
    pub fn push_to(&mut self, option: Key, rule: Rule) -> Option<()> {
        self.map.get_mut(&option)?.push(rule);

        Some(())
    }

    /// Empties the rules vector for key `option`.
    pub fn empty_vec(&mut self, option: Key) -> Option<()> {
        self.map.get_mut(&option)?.clear();

        Some(())
    }

    /// Returns the vector of rules for a given `ActionOption`.
    pub fn get(&self, option: &Key) -> Option<&Vec<Rule>> {
        self.map.get(option)
    }

    /// Returns the number of rules for a given `ActionOption`, or returns an error if the option is invalid.
    pub fn len_of(&self, option: &Key) -> Option<usize> {
        Some(self.map.get(option)?.len())
    }

    pub fn validate_option(&self, option: ActionOption) -> bool {
        match self.map.get(&option) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn contains(&self, rule: Rule) -> bool {
        for (_option, rules) in self.map.iter() {
            if rules.contains(&rule) {
                return true;
            }
        }

        false
    }

    pub fn remove(&mut self, rule: Rule) -> Option<Rule> {
        for (_option, rules) in self.map.iter_mut() {
            if rules.contains(&rule) {
                let index = rules.iter().position(|r| r == &rule).unwrap();
                return Some(rules.remove(index));
            }
        }

        None
    }
}
