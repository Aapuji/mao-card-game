/* Created by Om Sharma
 * 
 * This "Player" struct represents a player in the game. It
 * holds the name of the player and their hand (of cards).
 * It will be able to draw a card, play a card, and more.
 */

use crate::card::Card;
use crate::deck::Deck;

/// A struct representing a player in the game.
#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    /// Creates a new `Player` with name `name` and hand as an empty `Vec`.
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: Vec::new(),
        }
    }

    /// Draws 1 card from `deck`. If `deck` is empty, it creates a new `Deck` and appends it to `deck`, then draws from it.
    pub fn draw(&mut self, deck: &mut Deck) -> Card {
        deck.deal(1, self)[0]
    }

    /// Plays the card at `card_index` from hand, and puts it on the top of `deck`.
    ///
    /// Precondition: 0 <= card_index < self.hand.len()
    pub fn play_card(&mut self, card_index: usize, deck: &mut Deck) {
        let card = self.hand.swap_remove(card_index);
        // println!("New Card @ Pos: {}", self.hand[card_index]); This line breaks when card_index = self.hand.len()-1, also no idea what it was supposed to do in the first place
        deck.push_top(card);
    }

    pub fn clear_hand(&mut self) {
        self.hand.drain(0..);
    }

    /// Returns this player's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns a mutable reference to this player's hand.
    pub fn mut_hand(&mut self) -> &mut Vec<Card> {
        &mut self.hand
    }

    /// Returns an immutable reference to this player's hand.
    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }

    /// Returns the number of cards in this player's hand.
    pub fn num_cards(&self) -> usize {
        self.hand.len()
    }
}

/// Initializes a `Vec` of `Player`s, given a list of their names as arguments.
#[macro_export]
macro_rules! players {
    () => {
        vec![]
    };

    ( $( $name:expr ),* $(,)?) => {
      vec![$(
        mao::player::Player::new(
          String::from($name)
        )
      ),*]
    }
}
