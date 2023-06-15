/* Created by Skylar Huber
 *
 * This file is responsible for providing rendering for
 * the start screen.
 */

use super::engine::{RenderResult, Screen, TextFrameBuffer};
use crate::game::Game;
use rand::seq::SliceRandom;

use super::engine::RenderableElement;
use super::img::Img;

#[derive(Debug)]
pub struct TitleScreen {
    pub show_instructions: bool,
}

impl Screen for TitleScreen {
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer, _game: Option<&Game>) -> RenderResult<()> {
        Img::from_str(include_str!("images/title_screen")).render(fb, 0, 0)?;
        fb.text_wrapped(splash(), 5, 5, fb.width() - 10)?;

        if self.show_instructions {
          let img = Img::from_str(include_str!("images/instructions_screen"));
          img.render(
            fb,
            fb.width().saturating_sub(img.max_width())/2, 
            fb.height().saturating_sub(img.height())/2,
          )?;
        }

      Ok(())
    }
}

fn splash() -> &'static str {
    &[
        "The only card game about being confused.",
        "Wikipedia says this game is about Mao Zedong's rule being chaotic.",
        "Gluten free",
        "猫\u{200D}猫\u{200D}猫\u{200D}猫\u{200D} :3c",
        "Forgot the rules? No problem, you weren't supposed to know them anyway.",
        "[Bottom Text]",
        "The game that makes fun of you for not knowing what you aren't supposed to know.",
        "Made in room 318",
    ][..]
        .choose(&mut rand::thread_rng())
        .unwrap()
}
