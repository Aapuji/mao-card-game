/* ***** Created by Om Sharma and Skylar Huber *****
 *
 * This program will simulate the card game, "Mao" in
 * the console. It willrender the title screens and
 * rules screens, then get the players , and runs the
 * game, handling any render errors.
 */

use mao::game::Game;
use mao::player::Player;
use mao::render::ansi::{ANSIColor, ANSI_STYLE_RESET};
use mao::render::engine::{RenderResult, Screen};
use mao::render::error_handling;
use mao::render::name_select_screen::select_names;
use mao::render::title_screen::TitleScreen;

//  _  _ ____ ____    ____ ____ _  _ ____
//  |\/| |__| |  |    | __ |__| |\/| |___
//  |  | |  | |__|    |__] |  | |  | |___ .
//
//  _   _ ____ _  _ . _    _       _    ____ ____ ____
//   \_/  |  | |  | ' |    |       |    |  | [__  |___
//    |   |__| |__|   |___ |___    |___ |__| ___] |___
//
//  ___  ____ ___
//  |__] |___  |
//  |__] |___  |
//

/// Runs tha game and handles render errors.
fn main() {
    if let Err(err) = main_r() {
        error_handling::print_render_error(err);
    }
}

/// Actually runs the game, and returns `Ok(())` if the game went well, or a `Err<RenderError>` if there was a rendering error.
fn main_r() -> RenderResult<()> {
    (TitleScreen {
        show_instructions: false,
    })
    .render_then_wait(None)?;
    (TitleScreen {
        show_instructions: true,
    })
    .render_then_wait(None)?;

    let players: Vec<_> = select_names()?.into_iter().map(Player::new).collect();

    let mut game = Game::new(players);
    println!("Rule Map: {:#?}", game.rule_map());

    println!(
        "{}Total Cards: {}{}",
        ANSIColor::LightGreen.fg(),
        game.total_cards(),
        ANSI_STYLE_RESET
    );

    game.play()?;

    Ok(())
}
