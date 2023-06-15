/* Created by Om Sharma
 * 
 * This program simulates a deck with the "Deck" struct,
 * which holds all of the cards in the deck. It will be
 * able to shuffle itself, deal cards, and more. 
 */

use crate::card::Card;
use crate::player::Player;
use enum_iterator::all;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};

/// A struct representing a deck.
/// Implemented as just a `VecDeque<mao::card::Card>`.
#[derive(Debug)]
pub struct Deck {
    cards: VecDeque<Card>, // upside-down stack (last element is last card in stack)
}

impl Deck {
    /// Creates a new `Deck` from `cards`.
    pub fn new(cards: VecDeque<Card>) -> Self {
        Self { cards }
    }

    /// Returns a default, unshuffled `Deck` of 52 `Card`s.
    pub fn default_52() -> Self {
        Self {
            cards: all::<Card>().collect::<VecDeque<_>>(),
        }
    }

    /// Returns an empty `Deck`.
    pub fn empty() -> Self {
        Self {
            cards: VecDeque::new(),
        }
    }

    /// Randomizes the order of `Card`s in `self`.
    pub fn shuffle(&mut self) {
        self.cards.make_contiguous().shuffle(&mut thread_rng());
    }

    /// Returns a reference to `self.cards`.
    pub fn cards(&self) -> &VecDeque<Card> {
        &self.cards
    }

    /// Returns the size of the deck (# of cards).
    pub fn size(&self) -> usize {
        self.cards.len()
    }

    /// Returns the top (front) `Card` of the deck.
    pub fn top(&self) -> Card {
        self[0]
    }

    /// Returns the bottom (back) `Card` of the deck.
    pub fn bottom(&self) -> Card {
        self[self.size() - 1]
    }

    /// Pushes `card` to the top of the deck.
    pub fn push_top(&mut self, card: Card) {
        self.cards.push_front(card);
    }

    /// Pushes `card` to the bottom of the deck.
    pub fn push_bottom(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    /// Appends all the cards in `cards`, which is an iterator of `Card`s, to the bottom (back) of the deck.
    pub fn append<I>(&mut self, cards: I)
    where
        I: Iterator<Item = Card>,
    {
        self.cards.extend(cards);
    }

    /// Prepends all the cards in `cards`, which is an iterator of `Card`s, to the top (front) of the deck.
    pub fn prepend<I>(&mut self, cards: I)
    where
        I: Iterator<Item = Card>,
    {
        for card in cards {
            self.cards.push_front(card);
        }
    }

    /// Attempts to deal `amt` cards to `player`'s hand. If there aren't enough cards in the deck, it returns `Err` with the number of cards it didn't deal.
    fn deal_fallible(&mut self, amt: usize, player: &mut Player) -> Result<Vec<Card>, usize> {
        if self.size() == 0usize {
            return Err(amt);
        }

        if amt > self.size() {
            let size = self.size();
            player.mut_hand().extend(self.cards.drain(0..));
            return Err(amt - size);
        }

        let drawn: Vec<Card> = self.cards.drain(0..amt).collect();
        player.mut_hand().extend(drawn.iter());
        Ok(drawn)
    }

    /// This deals cards, just like `deal_fallible`, but will append a new deck to the end if it cannot deal `amt` cards.
    pub fn deal(&mut self, amt: usize, player: &mut Player) -> Vec<Card> {
        self.check_size_and_append(amt);
        self.deal_fallible(amt, player).unwrap()
    }

    /// Prepends `amt` cards into `deck`. If `amt` is larger than `self.size()`, it still adds as many as it can (emptying this deck), but then it also returns `Err` with the number of cards it did not prepend.
    pub fn inject(&mut self, amt: usize, deck: &mut Deck) -> Result<(), usize> {
        if amt > self.size() {
            let size = self.size();
            self.inject(self.size(), deck).unwrap();
            return Err(amt - size);
        }

        deck.prepend(self.cards.drain(0..amt));
        Ok(())
    }

    /// Checks size of `pile`. If it's smaller than `cmp`, then it appends a randomized `Deck`.
    pub fn check_size_and_append(&mut self, cmp: usize) {
        if self.size() < cmp {
            let mut another = Deck::default_52();
            another.shuffle();
            self.append(another.into_iter());
        }
    }

    pub fn clear(&mut self) {
        self.cards.drain(0..);
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'t> IntoIterator for &'t Deck {
    type Item = &'t Card;
    type IntoIter = std::collections::vec_deque::Iter<'t, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}

impl From<VecDeque<Card>> for Deck {
    fn from(cards: VecDeque<Card>) -> Self {
        Self { cards }
    }
}

impl From<Vec<Card>> for Deck {
    fn from(cards: Vec<Card>) -> Self {
        Self {
            cards: VecDeque::from(cards),
        }
    }
}

impl Index<usize> for Deck {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl IndexMut<usize> for Deck {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cards[index]
    }
}
