use super::engine::{RenderResult, RenderableElement, Screen, TextFrameBuffer};
use crate::game::Game;

pub enum PlayScreen {
    PlayerTurn,
}

impl Screen for PlayScreen {
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer) -> RenderResult<()> {
        let game = Game::instance().lock().unwrap();
        match self {
            PlayScreen::PlayerTurn => {
                let game = game.as_ref().unwrap();
                let player = game.current_player();
                fb.text(format!("Your turn, {}", player.name()).as_str(), 0, 0)?;

                fb.text("Hand", 10, 3);
                for (i, card) in player.hand().iter().enumerate() {
                    card.render(fb, i * 4 + 10, 4)?;
                }
            }
        }
        Ok(())
    }
}
