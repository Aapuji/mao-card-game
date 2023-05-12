use mao::ansi::Ansi::{Green, Reset};
use mao::card::{Card, Suit, Value};
use mao::game::Game;
use mao::players;
use mao::render::engine::{RenderResult, RenderableElement, Screen, TextFrameBuffer};
use mao::render::play_screen::PlayScreen;

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
    let mut game = Game::new(players);

    println!("{}Total Cards: {}{}", Green, game.total_cards(), Reset);
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    // for i in 0..game.num_players() - 1 {
    //     // println!("Player {i}: {:#?}", game.current_player());
    //     game.next_player();
    // }

    // game.play();

    PlayScreen::PlayerTurn.render(Some(&game))?;

    Ok(())
}
