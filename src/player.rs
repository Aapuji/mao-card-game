use crate::card::Card;
use crate::deck::Deck;

#[derive(Debug)]
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

    pub fn draw(&mut self, deck: &mut Deck) -> Result<(), usize> {
        deck.deal(1, self)
    }

    // Precondition: 0 <= card_index < self.hand
    pub fn play_card(&mut self, card_index: usize, deck: &mut Deck) {
        let card = self.hand.swap_remove(card_index);
        deck.push_top(card);
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

    pub fn num_cards(&self) -> usize {
        self.hand.len()
    }
}

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
