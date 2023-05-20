use mao::game::Game;
use mao::player::Player;
use mao::render::engine::{RenderResult, Screen};
use mao::render::title_screen::TitleScreen;
use mao::render::ansi::{ANSIColor, ANSI_STYLE_RESET};
use mao::render::name_select_screen::select_names;
use mao::render::error_handling;
use mao::players;

//
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

// true theme song moment: https://open.spotify.com/track/1cKqgD7czbEuNpBU9uTs14?si=e80d694924424b0e

fn main() {
    if let Err(err) = main_r() {
        error_handling::print_render_error(err);
    }
}

fn main_r() -> RenderResult<()> {
    (TitleScreen {}).render_then_wait(None)?;

    // let players: Vec<_> = select_names()?.into_iter().map(Player::new).collect();
  
    let players = players!(
        "Bob",
        "Darth",
        "Alice",
        "Dumbledore",
        "Andor",
        "XxMrFancyu2xX",
        "Bob Ross",
        "Steve",);
      // TODO: automatically insert zero width characters after 2-width characters
      // Idk if it's possible to do that but I *am* stupid sooo no sé.
      /*  "我但猫\u{200D}\u{200D}\u{200D}",
        "Puro",
        "Henry Stickmin",
        "The lord of all that which is unholy and who has a really long name for line wrapping testing purposes and the like.",
        "Natani",
        "<insert name here>",
        "MumboJumbo",
        "Joe Biden",
        "Dark Brandon",
        "ur mom",
        "ur dad",
        "me",
        "We're no strangers to love/You know the rules and so do I (do I)/A full/commitment's what I'm thinking of/You wouldn't get this from any other guy/I just wanna tell you how I'm feeling/Gotta make you understand/Never gonna give you up/Never gonna let you down/Never gonna run around and desert you/Never gonna make you cry/Never gonna say goodbye/Never gonna tell a lie and hurt you/We've known each other for so long/Your heart's been aching, but you're too shy to say it (say it)/Inside, we both know what's been going on (going on)/We know the game and we're gonna play it/And if you ask me how I'm feeling/Don't tell me you're too blind to see/Never gonna give you up/Never gonna let you down/Never gonna run around and desert you/Never gonna make you cry/Never gonna say goodbye/Never gonna tell a lie and hurt you/Never gonna give you up/Never gonna let you down/Never gonna run around and desert you/Never gonna make you cry/Never gonna say goodbye/Never gonna tell a lie and hurt you/We've known each other for so long/Your heart's been aching, but you're too shy to say it (to say it)/Inside, we both know what's been going on (going on)/We know the game and we're gonna play it/I just wanna tell you how I'm feeling/Gotta make you understand/Never gonna give you up/Never gonna let you down/Never gonna run around and desert you/Never gonna make you cry/Never gonna say goodbye/Never gonna tell a lie and hurt you/Never gonna give you up/Never gonna let you down/Never gonna run around and desert you/Never gonna make you cry/Never gonna say goodbye/Never gonna tell a lie and hurt you/Never gonna give you up/Never gonna let you down/Never gonna run around and desert you/Never gonna make you cry/Never gonna say goodbye/Never gonna tell a lie and hurt you",
        "Beta",
    );*/
    let mut game = Game::new(players);
    println!("Rule Map: {:#?}", game.rule_map());

    println!("{}Total Cards: {}{}", ANSIColor::LightGreen.fg(), game.total_cards(), ANSI_STYLE_RESET);

  
    let winner_id = game.play()?;
    dbg!(winner_id);
    
  

    // let k = PlayScreen::Turn(PlayerScreenTurnState::Speak(
    //     false,
    //     Card::new(Value::Ace, Suit::Diamonds),
    // ))
    // .render_then_input(Some(&game))?;
    // let k = PlayScreen::Turn(PlayerScreenTurnState::Action)
    // .render_then_input(Some(&game))?;

    // let responses = k.split("."); // TODO: Maybe split on ".|"

    // println!("Input: {}\nRes: {:?}", k, responses);

    // PlayScreen::Mistake(format!("You idiot! You Bafoon. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua, Bottom Text."), Card::new(Value::Five, Suit::Clubs)).render(Some(&game))?;

    // std::thread::sleep(std::time::Duration::from_millis(2000));

    // TitleScreen {}.render(Some(&game))?;

    // PlayScreen::Mistake(format!("You idiot! You Bafoon. ivs l kjsv  ljds ldsf dfl jdf ldfji feoa ji fej afdo dajlf ifd jl dkl fdjkl afflad jka f ofijo djo fio elfj efl jl feal efj lefil efa jlefai lefajl efai feaj, Bottom Text. {k}"), Card::new(Value::Five, Suit::Clubs)).render(None)?;

    Ok(())
}
