use crate::card::{Card, Suit, Value};
use enum_iterator::all;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::iter::IntoIterator;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn default_52() -> Self {
        Self {
            cards: all::<Card>().collect::<Vec<_>>(),
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}
