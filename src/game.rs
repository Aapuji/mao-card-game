use crate::card::{Card, Value};
use crate::deck::Deck;
use crate::player::Player;
use crate::render::engine::{RenderResult, Screen};
use crate::render::play_screen::*;
use crate::rule::priority::{ActionOption, Priority};
use crate::rule::{rule_map::RuleMap, Action, Event, Rule};

/// The game control struct, representing the game itself, and containing all game info and state transitions.
#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    draw_pile: Deck,
    used_pile: Deck,
    order: Order,
    player_index: usize,
    game_over: bool,
    priority: Priority, // Order to apply rules
    rule_map: RuleMap,
}

// Note: Removed GAME_INSTANCE, because it looked like no matter
// what I did, there was guaranteed threadlock due to taking multiple
// mutable references simultaniously. We will need to pass a
// reference to game directly to game to all function calls that could
// need it.

impl Game {
    /// Creates a new `Game` instance given a vector of `Players`.
    pub fn new(players: Vec<Player>) -> Self {
        let mut game = Self {
            players,
            draw_pile: Deck::default_52(),
            used_pile: Deck::empty(),
            order: Order::Forward,
            player_index: 0,
            game_over: false,
            rule_map: RuleMap::default(),
            priority: Priority::default(),
        };

        // Deals 7 cards to each player
        // If there are less than 24 cards left, it adds another 52 cards to the deck
        game.draw_pile.shuffle();
        for player in game.players.iter_mut() {
            game.draw_pile
                .deal(1, player) // Remember to change this to 7!
                .expect("Should have been able to deal 7 cards to player");
            Game::check_size_and_append(&mut game.draw_pile, 24);
        }

        // Checks for 25 here because 1 card is put in used_pile
        Game::check_size_and_append(&mut game.draw_pile, 25);
        game.draw_pile.inject(1usize, &mut game.used_pile).unwrap();

        // Prepares default rules
        // Consecutive 7s rule isn't included here, so has to be done elsewhere
        // Also Ace rule
        // game.rule_map
        //     .push_to(
        //         ActionOption::Say,
        //         Rule::new(
        //             Event::ValuePlayed(Value::Seven),
        //             Action::Say(String::from("have a nice day")),
        //         ),
        //     )
        //     .expect("For some reason, `ActionOption::Say` is not a key.");
        // game.rule_map
        //     .push_to(
        //         ActionOption::Skip,
        //         Rule::new(Event::ValuePlayed(Value::Ace), Action::Skip),
        //     )
        //     .expect("For some reason, `ActionOption::Skip` is not a key.");
        // game.rule_map
        //     .push_to(
        //         ActionOption::Repeat,
        //         Rule::new(Event::ValuePlayed(Value::Two), Action::Repeat),
        //     )
        //     .expect("For some reason, `ActionOption::Repeat` is not a key.");
        // game.rule_map
        //     .push_to(
        //         ActionOption::Reverse,
        //         Rule::new(Event::ValuePlayed(Value::Eight), Action::Reverse),
        //     )
        //     .expect("For some reason, `ActionOption::Reverse` is not a key.");
        // game.rule_map
        //     .push_to(
        //         ActionOption::Wild,
        //         Rule::new(Event::ValuePlayed(Value::Jack), Action::Wild),
        //     )
        //     .expect("For some reason, `ActionOption::Wild` is not a key.");

        game.add_rule(Rule::new(
            Event::ValuePlayed(Value::Seven),
            Action::Say(String::from("have a nice day")),
        ));

        game.add_rule(Rule::new(Event::ValuePlayed(Value::Ace), Action::Skip));

        game.add_rule(Rule::new(Event::ValuePlayed(Value::Two), Action::Repeat));

        game.add_rule(Rule::new(Event::ValuePlayed(Value::Eight), Action::Reverse));

        game.add_rule(Rule::new(Event::ValuePlayed(Value::Jack), Action::Skip));

        // Starts game
        println!("Commence");

        game
    }

    /// Checks size of `pile`. If it's smaller than `cmp`, then it appends a randomized `Deck`.
    pub fn check_size_and_append(pile: &mut Deck, cmp: usize) {
        if pile.size() < cmp {
            let mut another = Deck::default_52();
            another.shuffle();
            pile.append(another.into_iter());
        }
    }

    /// Advances the current player to the next one, and returns an immutable reference to it.
    ///
    /// Follows the direction of play. Eg. if the game is moving in the `Backward`s direction, then it goes backwards.
    pub fn next_player(&mut self) -> &Player {
        self.player_index = match self.order {
            Order::Forward => {
                if self.player_index == self.num_players() - 1 {
                    0
                } else {
                    self.player_index + 1
                }
            }

            Order::Backward => {
                if self.player_index == 0 {
                    self.num_players() - 1
                } else {
                    self.player_index - 1
                }
            }
        };

        println!("NEXT PLAYER: {}", self.player_index);

        &self.players[self.player_index]
    }

    /// Checks `quotes` to see if they all follow the required values for `reqs`. If each `quote` can be matched with a `req`, then it returns `Ok(())`, otherwise it returns an `Err` with a tuple with the "Incorrect use of _"s first and "Failure to say _"s second: `Err((Incorrects, Failures))`.
    pub fn check_quotes(
        &self,
        quotes: &Vec<String>,
        reqs: &Vec<String>,
    ) -> Result<(), (Vec<String>, Vec<String>)> {
        if quotes.is_empty() && reqs.is_empty() {
            return Ok(());
        }

        let mut new_quotes = quotes.clone(); // Remaining quotes are "Incorrect Use of _."
        let mut new_reqs = reqs.clone(); // Remaining reqs are "Failure to Say _."

        let mut i = 0;
        while !new_reqs.is_empty() {
            while i < new_quotes.len() {
                if new_quotes[i] == new_reqs[i] {
                    new_reqs.remove(i);
                    new_quotes.remove(i);
                    break;
                }

                i += 1;
            }
        }

        if new_quotes.is_empty() && new_reqs.is_empty() {
            Ok(())
        } else {
            Err((new_quotes, new_reqs))
        }
    }

    /// Attempts to add (or remove) a rule from `self.rule_map`. Returns Ok(()) if it works, Err(()) if the rule already exists, or there is a conflicting rule.
    /// A conflicting rule is a rule that has the same action
    pub fn add_rule(&mut self, rule: Rule) -> AddingRuleResult<()> {
        let action_option = ActionOption::from(rule.action());

        // Checks if `action_option` is a valid key
        if let Some(rules) = self.rule_map.get(&action_option) {
            // Gets all options of values in corresponding vec.
            let options = rules
                .iter()
                .map(|rule| ActionOption::from(rule.action()))
                .collect::<Vec<_>>();

            // If `action_option` is unique, push it to the vec.
            if !options.contains(&action_option) {
                self.rule_map.push_to(action_option, rule);
                Ok(())
            // If it is `Say(msg)`, check if `msg` is unique.
            } else if action_option == ActionOption::Say {
                // If not, return error.
                if rules
                    .iter()
                    .map(|r| r.action())
                    .collect::<Vec<_>>()
                    .contains(&rule.action())
                {
                    Err(AddingRuleError::ConflictingAction)
                // If so, push it; requiring many messages to be written.
                } else {
                    self.rule_map.push_to(action_option, rule);
                    Ok(())
                }
            // Otherwise, return error.
            } else {
                Err(AddingRuleError::ConflictingAction)
            }
        // Otherwise, return error.
        } else {
            Err(AddingRuleError::InvalidAction)
        }
    }

    /// Applies the rules in order of the priority
    pub fn apply_rules(&mut self, quotes: &Vec<String>) {
        for option in self.priority.iter() {
            // Gets all the rules that apply for the top card of `used_pile`.
            let rules = if let Some(rules) = self.rule_map.get(option) {
                rules
            } else {
                continue;
            }
            .iter()
            .filter(|rule| rule.event().arg_matches(self.used_pile[0]));

            match option {
                // If the option is `Say`, check quotes against required msgs.
                ActionOption::Say => {
                    let checked_quotes = self.check_quotes(quotes, {
                        &rules
                            .map(|rule| match rule.action() {
                                Action::Say(msg) => msg,
                                _ => String::new(), // Should not happen
                            })
                            .collect::<Vec<String>>()
                    });

                    match checked_quotes {
                        Ok(()) => (),
                        Err((incos, fails)) => {
                            eprintln!("Incorrect use of {:?}. Failure to use {:?}.", incos, fails)
                        }
                    }
                }

                ActionOption::Wild => todo!(),

                ActionOption::Draw => {
                    self.players[self.player_index].draw(&mut self.draw_pile);
                }

                ActionOption::Repeat => todo!(),

                ActionOption::Reverse => self.order = self.order.flip(),

                ActionOption::Skip => todo!(), // Just do `next_player?`
            }
        }
    }

    /// Goes through gameplay loop until a player wins, returns ???
    /// Currently in progress, and todo. Will probably work on after display is finished
    pub fn play(&mut self) -> RenderResult<usize> {
        /*
          Loop through each player
            Player chooses between:
              - playing card to used_pile
                Chooses a card from their hand to play <in Player>
              - taking card from draw_pile
                Draws card from pile <in Player>
        */
        while !self.game_over {
            self.screen_turn_start()?;

            let (did_draw, action_card) = if let Some(play) = self.screen_request_card_play()? {
                // Player played a card out of their hand.
                let action_card = self.current_player().hand()[play];
                self.players[self.player_index].play_card(play, &mut self.used_pile);
                (false, action_card)
            } else {
                // Player drew a card.
                self.players[self.player_index].draw(&mut self.draw_pile);
                (true, self.current_player().newest_card().unwrap())
            };

            // TODO : apply rules

            // Player is given a chance to speak
            let player_says = self.screen_request_turn_speak(did_draw, action_card)?;

            // TODO : apply rules

            if self.current_player().num_cards() == 0 {
                self.game_over = true;
                break;
            }

            self.next_player();
        }
        let winner = self.player_index;
        self.screen_win(winner)?;
        Ok(winner)
    }

    /// Utility UI function that requests notifies about change of turn.
    fn screen_turn_start(&self) -> RenderResult<()> {
        PlayScreen::NewTurn.render_then_wait(Some(self))
    }
    /// Utility UI function that requests notifies about change of turn.
    fn screen_win(&self, winner: usize) -> RenderResult<()> {
        PlayScreen::Win { winner }.render_then_wait(Some(self))
    }

    /// Utility UI function that requests their move action
    /// for their turn.
    ///
    /// Returns `Some(usize)` representing the card they play,
    /// or `None` if they draw from the deck.
    fn screen_request_card_play(&self) -> RenderResult<Option<usize>> {
        loop {
            let card_id = PlayScreen::Turn(TurnState::Action).render_then_input(Some(self))?;
            let card_id = card_id.as_str().trim();

            match card_id {
                "D" | "d" => {
                    return Ok(None);
                }
                _ => {
                    if let Ok(n) = card_id.parse::<usize>() {
                        if n >= 1 && n <= self.current_player().num_cards() {
                            return Ok(Some(n - 1));
                        }
                    }
                }
            }
        }
    }

    /// Utility UI function that requests their speaking action
    /// for their turn.
    ///
    /// Player's response is formatted "answer a. answer b."
    ///
    /// Returns `Vec<String>` representing the things they say.
    fn screen_request_turn_speak(
        &self,
        did_draw: bool,
        action_card: Card,
    ) -> RenderResult<Vec<String>> {
        Ok(PlayScreen::Turn(TurnState::Speak(did_draw, action_card))
            .render_then_input(Some(self))?
            .as_str()
            .split(".")
            .map(|v| v.to_lowercase().trim().to_string())
            .filter(|v| !v.is_empty())
            .collect())
    }

    /// Returns an immutable reference to the current player.
    pub fn current_player(&self) -> &Player {
        &self.players[self.player_index]
    }

    /// Returns a mutable reference to the current player.
    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.player_index]
    }

    /// Flips the order of play. `Order::Forward` becomes `Order::Backward` and vice versa.
    pub fn flip_order(&mut self) {
        self.order = self.order.flip();
    }

    /// Returns an immutable reference to the players in the game.
    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    /// Returns the number of players.
    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    /// Returns an immutable reference to the draw-card pile.
    pub fn draw_pile(&self) -> &Deck {
        &self.draw_pile
    }

    /// Returns an immutable reference to the used-cards pile.
    pub fn used_pile(&self) -> &Deck {
        &self.used_pile
    }

    /// Returns whether or not the game is over.
    pub fn game_over(&self) -> bool {
        self.game_over
    }

    /// Returns the map of rules. (For debugging purposes).
    pub fn rule_map(&self) -> &RuleMap {
        &self.rule_map
    }

    /// Returns the total number of cards in the game. This is calculated by the number of cards in the used pile + draw pile + size of each player's hand.
    pub fn total_cards(&self) -> u32 {
        let mut count = 0u32;
        for player in &self.players {
            count += player.hand().len() as u32;
        }

        count += self.draw_pile.size() as u32;
        count += self.used_pile.size() as u32;

        count
    }
}

/// An enum representing the possible orders of play.
#[derive(Debug)]
pub enum Order {
    Forward,
    Backward,
}

impl Order {
    /// Flips the order. `Forward` becomes `Backward`, and vice versa.
    fn flip(&self) -> Self {
        match self {
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }

    /// Returns a value for the order. `Order::Forward`: 1, `Order::Backward`: -1
    fn val(&self) -> isize {
        match self {
            Self::Forward => 1,
            Self::Backward => -1,
        }
    }
}

#[derive(Debug)]
pub enum AddingRuleError {
    InvalidAction,
    ConflictingAction,
}

pub type AddingRuleResult<T> = Result<T, AddingRuleError>;
