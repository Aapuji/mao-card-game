use mao::card::{Card, Suit, Value};
use mao::deck::Deck;
use mao::game::Game;
use mao::players;
use mao::render::engine::{BoxDrawingProfile, TextFrameBuffer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello World");

    // let c = Card::new(Value::Ace, Suit::Spades);

    // println!("{}", c);

    // let mut deck = Deck::default_52();
    // println!("{:#?}; {}", deck, deck.size());
    // deck.shuffle();
    // println!("SHUFFLED: {:#?}; {}", deck, deck.size());

    // let mut fb = TextFrameBuffer::new()?;

    // fb.fill_box(BoxDrawingProfile::SHADING[2], 2, 1, 4, 4)?;
    // fb.outline_box(BoxDrawingProfile::Normal, 2, 1, 4, 4)?;

    // println!("{}", fb.to_string());
    let players = players!("Bob", "Darth", "Alice");
    let GAME = Game::new(players);

    println!("{:#?}", &GAME);

    println!("{}", GAME.total_cards());

    Ok(())
}
