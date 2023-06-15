/* Created by Skylar Huber
 * 
 * This file is responsible for rendering all the gameplay screens.
 */

use super::ansi::ANSIColor;
use super::engine::{RenderError, RenderResult, RenderableElement, Screen, TextFrameBuffer};
use super::img::Img;
use crate::card::{Card, RenderableCard};
use crate::game::Game;
use rand::seq::SliceRandom;

/// The screen that represents any screen shown in play.
#[derive(Debug)]
pub enum PlayScreen {
    /// New round screen.
    NewRound { round_n: usize },
    /// Screen prompting the last winner to add a rule.
    CreateRule {
        winner: usize,
        state: CreateRuleState,
        format_issue: Option<String>,
    },
    /// Screen notifying the players of an automatic rule action, so it's obvious.
    RuleInvocation(RuleActionResult),
    /// New player screen, prompt them to hide their hand from opponents.
    NewTurn,
    /// Main turn screen, allows player to draw and play cards then speak.
    Turn(TurnState),
    /// Screen shown whenever the player violates a rule.
    Mistake {
        /// Things that should not have been said.
        incs: Vec<String>,
        /// ... in this round
        current_incs: usize,
        /// Things that should have been said.
        fails: Vec<String>,
        /// ... in this round
        current_fails: usize,
        /// Cards drawn as punishment.
        drawn: Vec<Card>,
        /// If you were given back your own card due to error.
        card_error: bool,
        /// ... in this round
        current_card_error: bool,
    },
    /// Screen shown at the end of the game when there's a winner.
    Win {
        /// This is a player index.
        winner: usize,
    },
}

/// The substate of PlayScreen::Turn
#[derive(Debug)]
pub enum TurnState {
    /// The "action" phase of a turn, where player draws into or plays from their hand
    Action,
    /// The "speak" phase of a turn, where player says something in response to their actions
    Speak(bool, Card),
}

#[derive(Debug)]
pub enum CreateRuleState {
    MakeEvent,
    MakeAction,
    RuleExists,
}
#[derive(Debug)]
pub enum RuleActionResult {
    Draw(Card),
    Skip { who: usize },
    Reverse,
    Repeat { who: usize },
}

impl Screen for PlayScreen {
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer, game: Option<&Game>) -> RenderResult<()> {
        let game = game.expect("PlayScreen may only be used if the game exists!");

        match self {
            PlayScreen::NewRound { round_n } => {
                // Draw the big text graphic.
                let img = Img::from_str(include_str!("images/next_round"));
                img.render(fb, fb.width().saturating_sub(img.max_width()) / 2, 1)?;

                let round_text = format!("ROUND #{round_n}");
                fb.text(
                    round_text.as_str(),
                    fb.width().saturating_sub(round_text.chars().count()) / 2,
                    5,
                )?;
            }
            PlayScreen::NewTurn => {
                // Draw the big text and arrow graphic.
                Img::from_str(include_str!("images/next_player")).render(fb, 0, 0)?;

                // Text that says who's turn it is.
                let player = game.current_player().name();
                fb.text_wrapped(
                    format!("Your turn, {player}! Make sure nobody else is watching your hand!")
                        .as_str(),
                    0,
                    5,
                    fb.width(),
                )?;
            }
            PlayScreen::Win { winner } => {
                // The "game over" ascii art text.
                Img::from_str(include_str!("images/game_over_text")).render(fb, 0, 0)?;

                // TODO: select one of many end screens
                let (center, img_src) = [
                    (true, include_str!("images/game_over/brandon")),
                    (true, include_str!("images/game_over/lookinside")),
                    (false, include_str!("images/game_over/stand")),
                    (true, include_str!("images/game_over/doge")),
                    (true, include_str!("images/game_over/rust")),
                    (true, include_str!("images/game_over/bobross")),
                ][..]
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                let img = Img::from_str(img_src);
                img.render(
                    fb,
                    if *center {
                        fb.width().saturating_sub(img.max_width()) / 2
                    } else {
                        0
                    },
                    7,
                )?;

                // Text that says who won.
                let winner = game.players()[*winner].name();
                fb.text_wrapped(
                    format!("{winner} wins or something").as_str(),
                    0,
                    5,
                    fb.width(),
                )?;
            }
            PlayScreen::CreateRule {
                winner,
                state,
                format_issue,
            } => {
                // Draw the big text graphic.
                let img = Img::from_str(include_str!("images/new_rule_text"));
                img.render(fb, fb.width().saturating_sub(img.max_width()) / 2, 1)?;

                // Text that says who's turn it is.
                let player = game.players()[*winner].name();
                fb.text_wrapped(
                    format!("Winner \"{player}\" makes new rule! Add a new rule to the game, and make sure nobody's watching!")
                        .as_str(),
                    0,
                    5,
                    fb.width(),
                )?;

                if let Some(format_issue) = format_issue {
                    fb.text_wrapped(format_issue.as_str(), 0, 6, fb.width())?;
                    fb.style_fg_box(ANSIColor::Red, 0, 6, fb.width(), 1)?;
                }

                match state {
                    CreateRuleState::MakeEvent => {
                        fb.text_wrapped(
                            "Select the condition the rule activates on...",
                            0,
                            7,
                            fb.width(),
                        )?;
                        fb.text_wrapped("Format:", 0, 9, fb.width())?;
                        fb.text_wrapped("║ suit is <suit>", 0, 10, fb.width())?;
                        fb.text_wrapped("║ value is <value>", 0, 11, fb.width())?;
                        fb.text_wrapped("║ card is <value> of <suit>", 0, 12, fb.width())?;
                        fb.text_wrapped("Example: \"card is 4 of spades\"", 0, 14, fb.width())?;
                        fb.set_input_prompt(format!("Enter rule event:"));
                    }
                    CreateRuleState::MakeAction => {
                        fb.text_wrapped("Select the action the rule takes...", 0, 7, fb.width())?;
                        fb.text_wrapped("Format:", 0, 9, fb.width())?;

                        fb.text_wrapped("║ draw", 0, 10, fb.width())?;
                        fb.text_wrapped("║ repeat", 0, 11, fb.width())?;
                        fb.text_wrapped("║ reverse", 0, 12, fb.width())?;
                        fb.text_wrapped("║ skip", 0, 13, fb.width())?;
                        fb.text_wrapped("║ say <text>", 0, 14, fb.width())?;
                        fb.text_wrapped("[note: \"{value}\" \"{card}\" and \"{suit}\" in <text> will be replaced to match the played card. ex. \"hello {value}\" -> \"hello four\"]", 0, 15, fb.width())?;
                        fb.style_fg_box(ANSIColor::LightBlack, 0, 15, fb.width(), 2)?;

                        fb.text_wrapped("Example: \"draw\"", 0, 18, fb.width())?;
                        fb.text_wrapped("Example: \"say hello world\"", 0, 19, fb.width())?;
                        fb.text_wrapped("Example: \"say I played {card}\"", 0, 20, fb.width())?;

                        fb.set_input_prompt(format!("Enter rule action:"));
                    }
                    CreateRuleState::RuleExists => {
                        fb.style_fg_box(ANSIColor::Red, 0, 7, fb.width(), 2)?;
                        fb.text_wrapped(
                            "An identical rule already exists, try again.",
                            0,
                            7,
                            fb.width(),
                        )?;
                    }
                };
            }
            PlayScreen::RuleInvocation(action) => {
                // Draw the big text graphic.
                let img = Img::from_str(include_str!("images/rule_invoked_text"));
                img.render(fb, fb.width().saturating_sub(img.max_width()) / 2, 1)?;

                match action {
                    RuleActionResult::Draw(card) => {
                        fb.text_wrapped(
                            format!("{} draws a card", game.current_player().name()).as_str(),
                            2,
                            5,
                            fb.width() - 4,
                        )?;
                        RenderableCard::Front(*card).render(fb, 2, 7)?;
                    }
                    RuleActionResult::Repeat { who } => {
                        fb.text_wrapped(
                            format!("{} gets an extra turn", game.players()[*who].name()).as_str(),
                            2,
                            5,
                            fb.width() - 4,
                        )?;
                    }
                    RuleActionResult::Reverse => {
                        fb.text_wrapped("turn order has reversed", 2, 5, fb.width() - 4)?;
                    }
                    RuleActionResult::Skip { who } => {
                        fb.text_wrapped(
                            format!("{} has been skipped", game.players()[*who].name()).as_str(),
                            2,
                            5,
                            fb.width() - 4,
                        )?;
                    }
                }
            }
            PlayScreen::Turn(state) => {
                // Nice little background graphic in the corner.
                Img::from_str(include_str!("images/bg")).render(
                    fb,
                    fb.width() - 64 / 2,
                    fb.height() - 64 / 4,
                )?;

                // Remind plyers who's turn it is.
                let player = game.current_player();
                fb.text_wrapped(
                    format!("Your turn, {}", player.name()).as_str(),
                    0,
                    0,
                    fb.width(),
                )?;

                // Draw the contents of their hand.
                fb.text("Hand", 10, 3)?;
                const DX: usize = 4;
                let n_hand_cols = (fb.width() - 13) / DX;

                for (i, card) in player.hand().iter().enumerate() {
                    let rend = RenderableCard::Front(*card);
                    let ix = i % n_hand_cols;
                    let iy = i / n_hand_cols;

                    if iy >= 3 {
                        // no more than three rows
                        let text = format!("... total of {} cards", player.hand().len());
                        let y = (RenderableCard::H + 1) * 3 + 5;
                        fb.text(
                            text.as_str(),
                            10,
                            y,
                        )?;
                        fb.style_clear_color_box(10, y, text.chars().count(), 1)?;
                        break;
                    }

                    let dy = iy * (RenderableCard::H + 1);

                    rend.render(fb, ix * DX + 10, 4 + dy)?;
                  let text = format!("{}", i + 1);
                    let x = ix * DX + 12;
                  let y = 4 + RenderableCard::H + dy;
                    fb.text(
                        text.as_str(),
                        x,
                        y,
                    )?;
                  fb.style_clear_color_box(x, y, text.chars().count(), 1)?;
                }

                // Draw an icon for the draw pile.
                fb.text("Draw", 2, 3)?;
                RenderableCard::Back.render(fb, 2, 4)?;
                fb.text("D", 4, 4 + RenderableCard::H)?;

                // Show the current top of the played deck.
                fb.text("Top", 3, 3 + RenderableCard::H * 2 + 2)?;
                RenderableCard::Back.render(fb, 2, 4 + RenderableCard::H * 2 + 2)?;
                RenderableCard::Front(game.used_pile().cards()[0]).render(
                    fb,
                    3,
                    4 + RenderableCard::H * 2 + 2,
                )?;

                match state {
                    // Action turns, player should draw or play.
                    TurnState::Action => fb.set_input_prompt(format!(
                        "D → Draw From Deck, 1-{} → Play Card",
                        player.hand().len()
                    )),
                    // Speak turn, player is prompted to speak.
                    TurnState::Speak(did_draw, card) => {
                        // Remind the user what they did.
                        fb.text(
                            if *did_draw { "You Drew" } else { "You Played" },
                            2,
                            fb.height() - RenderableCard::H - 2,
                        )?;
                        // Show what card they played or drew.
                        RenderableCard::Front(*card).render(
                            fb,
                            3,
                            fb.height() - RenderableCard::H - 1,
                        )?;

                        // Update the top of the deck if the card was played to show what
                        // would be visible before playing the card (in the code the play
                        // has already happened)
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
            PlayScreen::Mistake {
                incs,
                current_incs,
                fails,
                current_fails,
                drawn,
                card_error,
                current_card_error,
            } => {
                // Say who is the one who made the mistake.
                let name = game.current_player().name();
                fb.text_wrapped(
                    format!(" :: {name} made a mistake! ::").as_str(),
                    0,
                    0,
                    fb.width(),
                )?;
                fb.style_fg_box(ANSIColor::Red, 0, 0, fb.width(), 1)?;

                // Show a little splash text to make them feel bad.
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
                fb.style_fg_box(ANSIColor::LightBlack, 0, 1, fb.width(), 1)?;

                // Limit the number of errors shown so it does not overflow off the screen.
                let max_n = (fb.height() - (RenderableCard::H + 7)) / 2;
                let incs = &incs[incs.len().saturating_sub(max_n)..];
                let fails = &fails[fails.len().saturating_sub(max_n)..];

                // Alias some variables before drawing.
                let half_width = fb.width() / 2;
                let n_incs = incs.len();
                let n_fails = fails.len();

                // Draw all "card error" lines.
                if *card_error {
                    fb.text_wrapped(
                        format!("- Incorrect play").as_str(),
                        half_width,
                        1,
                        half_width - 1,
                    )?;

                    // Highlight the failures just added.
                    if *current_card_error {
                        fb.style_bg_box(ANSIColor::Red, half_width, 1, half_width, 2)?;
                    } else {
                        fb.style_clear_color_box(half_width, 1, half_width, 2)?;
                    }
                }

                // Draw all "incorrect usage" lines
                for (i, inc) in incs.into_iter().enumerate() {
                    fb.text_wrapped(
                        format!("- Incorrect use of \"{}\".", inc).as_str(),
                        0,
                        3 + 2 * i,
                        half_width - 1,
                    )?;

                    // Highlight the failures just added.
                    if n_incs - i <= *current_incs {
                        fb.style_bg_box(ANSIColor::Red, 0, 3 + 2 * i, half_width, 2)?;
                    }
                }

                // Draw all "failed to say" lines
                for (i, fail) in fails.into_iter().enumerate() {
                    fb.text_wrapped(
                        format!("- Failure to say \"{}\".", fail).as_str(),
                        half_width,
                        3 + 2 * i,
                        half_width - 1,
                    )?;

                    // Highlight the failures just added.
                    if n_fails - i <= *current_fails {
                        fb.style_bg_box(ANSIColor::Red, half_width, 3 + 2 * i, half_width, 2)?;
                    }
                }

                const H: usize = RenderableCard::H;

                // Display the card that was dealt back if applicable.
                let drawn = if *card_error && *current_card_error {
                    fb.text(
                        "You were redealt the card you played",
                        2,
                        fb.height() - H * 2 - 5,
                    )?;
                    RenderableCard::Front(drawn[0]).render(fb, 2, fb.height() - H * 2 - 4)?;
                    &drawn[1..]
                } else {
                    &drawn[..]
                };

                // Display all the cards that we just drew.
                fb.text(
                    format!("You were dealt {} card(s)", drawn.len()).as_str(),
                    2,
                    fb.height() - H - 3,
                )?;
                let mut n_left = drawn.len();
                for (i, card) in drawn.iter().enumerate().rev() {
                    let res =
                        RenderableCard::Front(*card).render(fb, 2 + 4 * i, fb.height() - H - 2);
                    match res.err() {
                        None => {}
                        Some(RenderError::DrawOutOfBounds(_, _, _)) => continue, // draw until we can't no more
                        Some(err) => return Err(err), // throw any other type of error
                    }
                    n_left -= 1;
                }
                // If we could not draw all the cards, say how many left there are unshown.
                if n_left > 0 {
                    let s = format!("...{n_left} more cards");
                    fb.text(
                        s.as_str(),
                        fb.width() - s.chars().count() - 1,
                        fb.height() - 1,
                    )?;
                }

                // Prompt for input.
                fb.set_input_prompt("Anything to say?".to_string());
            }
        }

        Ok(())
    }
}
