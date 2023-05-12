use super::engine::{RenderResult, RenderableElement, Screen, TextFrameBuffer};
use crate::card::RenderableCard;
use crate::game::Game;

pub enum PlayScreen {
    PlayerTurn,
}

impl Screen for PlayScreen {
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer, game: Option<&Game>) -> RenderResult<()> {
        match self {
            PlayScreen::PlayerTurn => {
                let game = game.as_ref().unwrap();
                let player = game.current_player();
                fb.text(format!("Your turn, {}", player.name()).as_str(), 0, 0)?;

                fb.text("Hand", 10, 3)?;
                for (i, card) in player.hand().iter().enumerate() {
                    let rend = RenderableCard::Front(card.clone());
                    rend.render(fb, i * 4 + 10, 4)?;
                    fb.text(format!("{}", i).as_str(), i * 4 + 12, 4 + RenderableCard::H)?;
                }

                fb.text("Draw", 2, 3)?;
                RenderableCard::Back.render(fb, 2, 4)?;
                fb.text("D", 4, 4 + RenderableCard::H)?;
            }
        }
        Ok(())
    }
}
