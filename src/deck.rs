use crate::card::Card;
use crate::player::Player;
use enum_iterator::all;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::iter::IntoIterator;

#[derive(Debug)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new(cards: VecDeque<Card>) -> Self {
        Self { cards }
    }

    pub fn default_52() -> Self {
        Self {
            cards: all::<Card>().collect::<VecDeque<_>>(),
        }
    }

    pub fn empty() -> Self {
        Self {
            cards: VecDeque::new(),
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.make_contiguous().shuffle(&mut thread_rng());
    }

    pub fn cards(&self) -> &VecDeque<Card> {
        &self.cards
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn append<I>(&mut self, cards: I)
    where
        I: Iterator<Item = Card>,
    {
        self.cards.extend(cards);
    }

    pub fn prepend<I>(&mut self, cards: I)
    where
        I: Iterator<Item = Card>,
    {
        for card in cards {
            self.cards.insert(0, card);
        }
    }

    /// Attempts to deal `amt` of cards to `player`'s hand. If there aren't enough cards in the deck, it returns `Err` with the number of cards it didn't deal.
    pub fn deal(&mut self, amt: usize, player: &mut Player) -> Result<(), usize> {
        if self.size() == 0usize {
            return Err(amt);
        }

        if amt == self.size() {
            player.mut_hand().extend(self.cards.drain(0..));
            return Ok(());
        }

        if amt > self.size() {
            let size = self.size();
            player.mut_hand().extend(self.cards.drain(0..));
            return Err(amt - size);
        }

        player.mut_hand().extend(self.cards.drain(0..amt));
        Ok(())
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
