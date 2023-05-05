use mao::card::{Card, Suit, Value};
use mao::deck::Deck;
use mao::game::Game;

fn main() {
    println!("Hello World");

    let c = Card::new(Value::Ace, Suit::Spades);

    println!("{}", c);

    let mut deck = Deck::default_52();
    println!("{:#?}; {}", deck, deck.size());
    deck.shuffle();
    println!("SHUFFLED: {:#?}; {}", deck, deck.size());
}
