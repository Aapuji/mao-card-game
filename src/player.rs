use crate::card::Card;

pub struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn mut_hand(&mut self) -> &mut Vec<Card> {
        &mut self.hand
    }

    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }
}
