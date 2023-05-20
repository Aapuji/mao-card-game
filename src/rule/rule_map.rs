use crate::rule::priority::ActionOption;
use crate::rule::Rule;
use enum_iterator::all;
use std::collections::HashMap;

/// The key type for a `RuleMap`.
type Key = ActionOption;

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

    /// Pushes `rule` to end of the vector at key `option`.
    pub fn push_to(&mut self, option: Key, rule: Rule) -> Result<(), &'static str> {
        self.map
            .get_mut(&option)
            .ok_or_else(|| "Invalid option, not in map.")?
            .push(rule);

        Ok(())
    }

    pub fn empty_vec(&mut self, option: Key) {
        self.map
            .get_mut(&option)
            .expect("Invalid option, not in map.")
            .clear()
    }

    pub fn get(&self, option: &Key) -> Option<&Vec<Rule>> {
        self.map.get(option)
    }

    pub fn len_of(&self, option: &Key) -> Option<usize> {
        Some(self.map.get(option)?.len())
    }
}
