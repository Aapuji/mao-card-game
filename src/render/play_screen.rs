use super::ansi::ANSIColor;
use super::engine::{RenderResult, RenderableElement, Screen, TextFrameBuffer};
use crate::card::{Card, RenderableCard};
use crate::game::Game;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum PlayScreen {
    NewTurn,
    Turn(TurnState),
    Mistake(String, Card),
    Win { winner: usize },
}

#[derive(Debug)]
pub enum TurnState {
    Action,
    Speak(bool, Card),
}

impl Screen for PlayScreen {
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer, game: Option<&Game>) -> RenderResult<()> {
        let game = game.expect("PlayScreen may only be used if the game exists!");

        match self {
            PlayScreen::NewTurn => {
                let player = game.current_player().name();

                for (i, ln) in [
                    r#" ::  _  _ ____ _  _ ___    ___  _    ____ _   _ ____ ____ "#,
                    r#" ::  |\ | |___  \/   |     |__] |    |__|  \_/  |___ |__/ "#,
                    r#" ::  | \| |___ _/\_  |     |    |___ |  |   |   |___ |  \ "#,
                    r#" ::                                                       "#,
                ]
                .iter()
                .enumerate()
                {
                    fb.text(&ln[..fb.width()], 0, i)?;
                }
                fb.text_wrapped(format!("Your turn, {player}!").as_str(), 0, 5, fb.width())?;
            }
            PlayScreen::Win { winner } => {
                let winner = game.players()[*winner].name();

                for (i, ln) in [
                    r#"____ ____ _  _ ____    ____ _  _ ____ ____   /"#,
                    r#"| __ |__| |\/| |___    |  | |  | |___ |__/  / "#,
                    r#"|__] |  | |  | |___    |__|  \/  |___ |  \ .  "#,
                    r#"                                              "#,
                ]
                .iter()
                .enumerate()
                {
                    fb.text(&ln[..fb.width()], 0, i)?;
                }

                fb.text_wrapped(
                    format!("{winner} wins or something").as_str(),
                    0,
                    5,
                    fb.width(),
                )?;
            }

            PlayScreen::Turn(state) => {
                let player = game.current_player();
                fb.text_wrapped(
                    format!("Your turn, {}", player.name()).as_str(),
                    0,
                    0,
                    fb.width(),
                )?;

                fb.text("Hand", 10, 3)?;
                const DX: usize = 4;
                let n_hand_cols = (fb.width() - 13) / DX;

                for (i, card) in player.hand().iter().enumerate() {
                    let rend = RenderableCard::Front(*card);
                    let ix = i % n_hand_cols;
                    let iy = i / n_hand_cols;

                    if iy >= 3 {
                        // no more than three rows
                        fb.text(
                            format!("... total of {} cards", player.hand().len()).as_str(),
                            10,
                            (RenderableCard::H + 1) * 3 + 5,
                        )?;
                        break;
                    }

                    let dy = iy * (RenderableCard::H + 1);

                    rend.render(fb, ix * DX + 10, 4 + dy)?;
                    fb.text(
                        format!("{}", i + 1).as_str(),
                        ix * DX + 12,
                        4 + RenderableCard::H + dy,
                    )?;
                }

                fb.text("Draw", 2, 3)?;
                RenderableCard::Back.render(fb, 2, 4)?;
                fb.text("D", 4, 4 + RenderableCard::H)?;

                fb.text("Top", 3, 3 + RenderableCard::H * 2 + 2)?;
                RenderableCard::Back.render(fb, 2, 4 + RenderableCard::H * 2 + 2)?;
                RenderableCard::Front(game.used_pile().cards()[0]).render(
                    fb,
                    3,
                    4 + RenderableCard::H * 2 + 2,
                )?;

                match state {
                    TurnState::Action => fb.set_input_prompt(format!(
                        "D → Draw From Deck, 1-{} → Play Card",
                        player.hand().len()
                    )),
                    TurnState::Speak(did_draw, card) => {
                        fb.text(
                            if *did_draw { "You Drew" } else { "You Played" },
                            2,
                            fb.height() - RenderableCard::H - 2,
                        )?;
                        RenderableCard::Front(*card).render(
                            fb,
                            3,
                            fb.height() - RenderableCard::H - 1,
                        )?;
                        if !did_draw {
                            RenderableCard::Front(game.used_pile().cards()[1]).render(
                                fb,
                                3,
                                4 + RenderableCard::H * 2 + 2,
                            )?;
                        }
                        fb.set_input_prompt(format!("Anything to say? (leave blank for silent)"))
                    }
                }
            }

            PlayScreen::Mistake(message, card) => {
                let name = game.current_player().name();
                fb.text_wrapped(
                    format!(" :: {name} made a mistake! ::").as_str(),
                    0,
                    0,
                    fb.width(),
                )?;
                fb.style_fg_box(ANSIColor::Red, 0, 0, fb.width(), 1)?;
                fb.text(
                    [
                        "You just mao'd wrong!",
                        "idot.",
                        "lol why didn't you know that rule?",
                        "I smell failure.",
                        "How could you??",
                        "Interrobang‽",
                        "you.getGood(\"loser\");",
                    ]
                    .choose(&mut rand::thread_rng())
                    .unwrap(),
                    0,
                    1,
                )?;

                fb.text_wrapped(message.as_str(), 0, 2, fb.width() - 1)?;

                const H: usize = RenderableCard::H;
                RenderableCard::Front(*card).render(fb, 2, fb.height() - H - 2)?;
                fb.text("You were dealt a card", 2, fb.height() - H - 3)?;

                fb.set_input_prompt("Anything to say?".to_string());
            }
        }

        Ok(())
    }
}
