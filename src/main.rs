use mao::card::{Card, Suit, Value};
use mao::game::Game;
use mao::players;
use mao::render::engine::{RenderResult, RenderableElement, Screen, TextFrameBuffer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rendering...");

    let players = players!(
        "Bob",
        "Darth",
        "Alice",
        "Dumbledore",
        "Andor",
        "XxMrFancyu2xX",
        "Bob Ross",
        "Steve",
    );
    Game::new(players);

    println!("Total Cards: {}", Game::instance().total_cards());

    for i in 0..game.num_players() - 1 {
        println!("Player {i}: {:#?}", Game::instance().current_player());
        Game::instance().next_player();
    }

    (TestScreen {}).render()?;

    Game::instance().play();
    Ok(())
}
