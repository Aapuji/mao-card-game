/* Created by Skylar Huber
 *
 * This file is responsible for asking the user if they'd like to play again.
 */

use super::engine::{RenderResult, Screen, TextFrameBuffer};
use crate::game::Game;

#[derive(Debug)]
pub struct PlayMoreConfirmScreen;

impl Screen for PlayMoreConfirmScreen {
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer, _game: Option<&Game>) -> RenderResult<()> {
        fb.set_input_prompt(format!("One more round (y/n)?"));
        Ok(())
    }
}
