/* Created by Om Sharma and Skylar Huber
 * 
 * This file is responsible for controlling game logic by
 * making calls to the render/input system and other game
 * components.
 * 
 */

use crate::card::{Card, Suit, Value};
use crate::deck::Deck;
use crate::player::Player;
use crate::render::engine::{RenderResult, Screen};
use crate::render::play_more_confirm_screen::PlayMoreConfirmScreen;
use crate::render::play_screen::*;
use crate::rule::priority::{ActionOption, Priority};
use crate::rule::{rule_map::RuleMap, Action, Event, Rule};
use serde::Serialize;
use tinytemplate::TinyTemplate;

/// The game control struct, representing the game itself, and containing all game info and state transitions.
#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    draw_pile: Deck,
    used_pile: Deck,
    order: Order,
    player_index: usize,
    round_over: bool,
    priority: Priority, // Order to apply rules
    rule_map: RuleMap,
}

impl Game {
    /// Creates a new `Game` instance given a vector of `Players`.
    pub fn new(players: Vec<Player>) -> Self {
        let mut game = Self {
            players,
            draw_pile: Deck::default_52(),
            used_pile: Deck::empty(),
            order: Order::Forward,
            player_index: 0,
            round_over: false,
            rule_map: RuleMap::default(),
            priority: Priority::default(),
        };

        // Checks for 25 here because 1 card is put in used_pile
        Deck::check_size_and_append(&mut game.draw_pile, 25);
        game.draw_pile.inject(1usize, &mut game.used_pile).unwrap();

        // Prepares default rules
        // Consecutive 7s rule isn't included here, so has to be done elsewhere
        game.add_rule(Rule::new(
            Event::ValuePlayed(Value::Seven),
            Action::Say(String::from("have a nice day")),
        ))
        .unwrap();

        game.add_rule(Rule::new(Event::ValuePlayed(Value::Ace), Action::Skip))
            .unwrap();

        game.add_rule(Rule::new(Event::ValuePlayed(Value::Two), Action::Repeat))
            .unwrap();

        game.add_rule(Rule::new(Event::ValuePlayed(Value::Eight), Action::Reverse))
            .unwrap();

        // Ace rule
        game.add_rule(Rule::new(
            Event::SuitPlayed(Suit::Spades),
            Action::Say(String::from("{value} of spades")),
        ))
        .unwrap();

        game
    }

    pub fn play(&mut self) -> RenderResult<Vec<usize>> {
        let mut winners = vec![];

        let mut round = 1;

        loop {
            PlayScreen::NewRound { round_n: round }.render_then_wait(Some(self))?;

            let winner = self.round()?;
            winners.push(winner);

            // Ask if they want to play again.
            if !self.confirm_next_round()? {
                break;
            }

            // Allow winner to add or remove a rule
            self.create_rule(winner)?;

            round += 1;
        }

        Ok(winners)
    }

    /// Goes through gameplay loop until a player wins, returns a result, with an `Ok` value holding the index of the winner.
    pub fn round(&mut self) -> RenderResult<usize> {
        // Resets values
        self.used_pile.clear();
        self.draw_pile = Deck::default_52();
        self.order = Order::Forward;
        self.player_index = 0;
        self.round_over = false;
      
        // Clear player's hands.
        for player in self.players.iter_mut() {
            player.clear_hand();
        }

        // Deals 7 cards to each player
        // If there are less than 24 cards left, it adds another 52 cards to the deck
        self.draw_pile.shuffle();
        for player in self.players.iter_mut() {
            self.draw_pile.deal(7, player);
        }

        // Checks for 25 here because 1 card is put in used_pile
        Deck::check_size_and_append(&mut self.draw_pile, 25);
        self.draw_pile.inject(1usize, &mut self.used_pile).unwrap();

        /*
          Loop through each player
            Player chooses between:
              - playing card to used_pile
                Chooses a card from their hand to play <in Player>
              - taking card from draw_pile
                Draws card from pile <in Player>
        */
        while !self.round_over {
            self.screen_next_player()?;
            /*
              `did_draw` is true if the player drew a card
              `action_card` is the target card of the action
                - the card they drew if they drew a card, or
                - the card they played when the played a card
            */
            let mut incorrect_play = false;
            let (did_draw, action_card) = if let Some(play) = self.screen_request_card_play()? {
                // Player played a card out of their hand.
                let action_card = self.current_player().hand()[play];
                incorrect_play = !self.validate_card_played(action_card);
                self.players[self.player_index].play_card(play, &mut self.used_pile);
                (false, action_card)
            } else {
                // Player drew a card.
                let card = self.players[self.player_index].draw(&mut self.draw_pile);
                (true, card)
            };

            // Player is given a chance to speak
            let quotes = self.screen_request_turn_speak(did_draw, action_card)?;

            println!("Qs: {:?}", &quotes);

            // Check for "thank you" if they drew a card.
            if did_draw {
                let mistakes = self.check_quotes(&quotes, &vec![String::from("thank you")]);

                println!("Mistakes: {:?}", mistakes); // Log check

                self.mistake_screen(mistakes, false, None)?;
            } else {
                println!("{}", self.used_pile[0]);

                let mut reqs = vec![];

                if self.players[self.player_index].num_cards() == 0 {
                    reqs.push(String::from("mao"));
                }

                let (mistakes, _idx) = self.apply_rules(&quotes, reqs, incorrect_play)?;
                self.mistake_screen(mistakes, incorrect_play, Some(action_card))?;
            }

            if self.current_player().num_cards() == 0 {
                self.round_over = true;
                break;
            }

            self.next_player();
        }

        let winner = self.player_index;
        self.screen_win(winner)?;
        Ok(winner)
    }

    pub fn confirm_next_round(&self) -> RenderResult<bool> {
        Ok(loop {
            let res = PlayMoreConfirmScreen.render_then_input(Some(&self))?;
            match res.trim().to_lowercase().as_str() {
                "y" | "yes" | "ok" | "play" | "continue" => break true,
                "n" | "no" | "quit" | "end" | "stop" => break false,
                _ => continue,
            }
        })
    }

    pub fn create_rule(&mut self, winner: usize) -> RenderResult<()> {
        loop {
            let mut format_issue = None;
            let event: Event = loop {
                let event_str = PlayScreen::CreateRule {
                    winner,
                    format_issue,
                    state: CreateRuleState::MakeEvent,
                }
                .render_then_input(Some(self))?;

                let (typ, data) = if let Some(x) = event_str.trim().split_once(" is ") {
                    x
                } else {
                    format_issue = Some(format!("'{}' is missing ' is '", event_str.trim()));
                    continue;
                };
                let typ = typ.trim().to_lowercase();
                let data = data.trim();

                if let Some(event) = match typ.as_str() {
                    "card" => {
                        if let Some((value_str, suit_str)) = data.split_once(" of ") {
                            if let (Some(value), Some(suit)) =
                                (Value::from_str(value_str), Suit::from_str(suit_str))
                            {
                                Some(Event::CardPlayed(Card::new(value, suit)))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    "suit" => {
                        if let Some(suit) = Suit::from_str(data) {
                            Some(Event::SuitPlayed(suit))
                        } else {
                            None
                        }
                    }
                    "value" => {
                        if let Some(value) = Value::from_str(data) {
                            Some(Event::ValuePlayed(value))
                        } else {
                            None
                        }
                    }
                    _ => None,
                } {
                    format_issue = Some("".to_string());
                    break event;
                } else {
                    format_issue = Some(format!("'{}' is invalid for type '{}'", data, typ));
                    continue;
                }
            };
            let action: Action = loop {
                let action_str = PlayScreen::CreateRule {
                    winner,
                    format_issue,
                    state: CreateRuleState::MakeAction,
                }
                .render_then_input(Some(self))?;

                match action_str.trim().to_lowercase().as_str() {
                    "draw" => break Action::Draw,
                    "repeat" => break Action::Repeat,
                    "reverse" => break Action::Reverse,
                    "skip" => break Action::Skip,
                    s => {
                        if s.starts_with("say ") {
                            // `s` is trimmed, so with the ending whitespace in "say ", it's known there will be non whitespace after that, and so it can safely be assumed that `quote` will be non-empty
                            let quote = (&s["say ".len()..]).trim().to_string();

                            if quote.contains(".") {
                                format_issue = Some(format!("'{}' may not contain '.'", &quote));
                                continue;
                            }

                            dbg!(&quote);
                            break Action::Say(quote);
                        } else {
                            format_issue = Some(format!("'{}' is invalid", s));
                        }
                    }
                }
            };

            if let Err(error) = self.add_rule(Rule::new(event, action)) {
                // TODO handle other kinds of error, (though I do beleive this is the only one that comes up)
                match error {
                    AddingRuleError::ConflictingAction => {
                        PlayScreen::CreateRule {
                            format_issue: None,
                            winner,
                            state: CreateRuleState::RuleExists,
                        }
                        .render_then_wait(Some(&self))?;
                    }
                    AddingRuleError::InvalidAction => {
                        panic!("SHOULD NOT HAPPEN");
                    }
                }
            } else {
                // all good, break
                break;
            }
        }
        Ok(())
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

    pub fn prev_player(&mut self) {
        self.flip_order();
        self.next_player();
        self.flip_order();
    }

    /// Checks `quotes` to see if they all follow the required values for `reqs` and returns the mistakes made. If each `quote` can be matched with a `req`, then it returns `None`, otherwise it returns `Some` with a tuple with the "Incorrect use of _"s first and "Failure to say _"s second: `Some((Incorrects, Failures))`.
    pub fn check_quotes(&self, quotes: &Vec<String>, reqs: &Vec<String>) -> Option<Mistakes> {
        if quotes.is_empty() {
            return if reqs.is_empty() {
                None
            } else {
                Some((vec![], reqs.clone()))
            };
        }

        let mut new_quotes = quotes.clone(); // Remaining quotes are "Incorrect Use of _."
        let mut new_reqs = reqs.clone(); // Remaining reqs are "Failure to Say _."

        println!("INSIDE CHECK QUOTES");

        // Loop through each requirement
        let mut i = 0i32;
        for req in reqs.iter() {
            // Loop through each quote and remove both the quote and corresponding requirement if they match.
            let mut j = 0;
            while j < new_quotes.len() {
                if &new_quotes[j] == req {
                    new_quotes.remove(j);
                    new_reqs.remove(i as usize);
                    i -= 1;
                    break;
                }

                j += 1;  
            }

            i += 1;
        }

        if new_quotes.is_empty() && new_reqs.is_empty() {
            None
        } else {
            Some((new_quotes, new_reqs))
        }
    }

    pub fn clear_rules(&mut self) {
        // for (option, _) in self.rule_map.iter
    }

    /// Attempts to add (or remove) a rule from `self.rule_map`. Returns Ok(()) if it works, Err(()) if the rule already exists, or there is a conflicting rule.
    /// A conflicting rule is a rule that has the same action
    pub fn add_rule(&mut self, rule: Rule) -> AddingRuleResult<()> {
        let action_option = ActionOption::from(rule.action());

        // Checks if `action_option` is a valid key
        // If so, sets `rules` to the vector of rules for that option.
        // All actions for each rule in `rules` are the same for one pass of the loop
        if let Some(rules) = self.rule_map.get(&action_option) {
            if rules.is_empty() {
                self.rule_map.push_to(action_option, rule).unwrap();
                return Ok(());
            }

            // Gets all options of values in corresponding vec.
            let events = rules.iter().map(|rule| rule.event()).collect::<Vec<_>>();

            // If the event that triggers the action is unique, push it to the vec.
            if !events.contains(&rule.event()) {
                self.rule_map.push_to(action_option, rule).unwrap();
                Ok(())
            // Otherwise, `rule`'s event is not unique.
            // If `rule`'s action is `Say(msg)`, check if `msg` is unique.
            } else if action_option == ActionOption::Say {
                // If not, return error.
                if rules
                    .iter()
                    .map(|r| r.action())
                    .collect::<Vec<_>>()
                    .contains(&rule.action())
                {
                    Err(AddingRuleError::ConflictingAction)
                // If so, push it; requiring many messages to be written by player.
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

    /// Applies the rules in order of the priority. Returns a tuple of mistakes and the player index of the player who just played.
    ///
    /// `req_msgs` is the initial reqs to check against `quotes`.
    pub fn apply_rules(
        &mut self,
        quotes: &Vec<String>,
        mut req_msgs: Vec<String>,
        was_invalid: bool,
    ) -> RenderResult<(Option<Mistakes>, usize)> {
        let idx = self.player_index;

        let priority = self.priority.clone();
        for option in priority.iter() {
            // Gets all the rules that apply for the top card of `used_pile`.
            let rules = if let Some(rules) = self.rule_map.get(option) {
                rules
            } else {
                continue;
            }
            .iter()
            .filter(|rule| rule.event().arg_matches(self.used_pile[0]))
            .collect::<Vec<_>>();

            // If no rules with this action, continue to next action.
            if rules.is_empty() {
                continue;
            }

            match (was_invalid, option) {
                // If option is `Say`, append required msgs to vector.
                (_, ActionOption::Say) => {
                    req_msgs.extend(rules.iter().map(|rule| match rule.action() {
                        Action::Say(msg) => Game::parse_message(msg, self.used_pile[0]),
                        _ => String::new(), // Should not happen
                    }));

                    println!("MSGS: {:?}", req_msgs);
                }

                (false, ActionOption::Draw) => {
                    let drawn_card = self.players[self.player_index].draw(&mut self.draw_pile);

                    PlayScreen::RuleInvocation(RuleActionResult::Draw(drawn_card))
                        .render_then_wait(Some(&self))?;
                }

                (false, ActionOption::Repeat) => {
                    let who_is_repeating = self.player_index;

                    self.prev_player();
                    PlayScreen::RuleInvocation(RuleActionResult::Repeat {
                        who: who_is_repeating,
                    })
                    .render_then_wait(Some(&self))?;
                }

                (false, ActionOption::Reverse) => {
                    self.order = self.order.flip();

                    PlayScreen::RuleInvocation(RuleActionResult::Reverse)
                        .render_then_wait(Some(&self))?;
                }

                (false, ActionOption::Skip) => {
                    self.next_player();

                    let who_is_skipped = self.player_index; // "current" player will be skipped by next `next_player` call
                    PlayScreen::RuleInvocation(RuleActionResult::Skip {
                        who: who_is_skipped,
                    })
                    .render_then_wait(Some(&self))?;
                } // Just do `next_player?`

                _ => {}
            }
        }

        // println!("Reqs: {}", &req_msgs[0]);
        // Check quotes and return the player index of the player who just played
        Ok((self.check_quotes(quotes, &req_msgs), idx))
    }

    fn parse_message(message: String, card: Card) -> String {
        let mut template = TinyTemplate::new();

        template.add_template("template", &message).unwrap();

        let context = Context {
            card: format!(
                "{} of {}",
                card.value().full_name(),
                card.suit().full_name()
            )
            .to_lowercase()
            .to_owned(),
            value: format!("{}", card.value().full_name())
                .to_lowercase()
                .to_owned(),
            suit: format!("{}", card.suit().full_name())
                .to_lowercase()
                .to_owned(),
        };

        let fmt_msg = template
            .render("template", &context)
            .expect("Should not panic here (@ formatting the message).");

        println!("Formatted Message: {}", &fmt_msg);

        fmt_msg
    }

    /// Given the mistakes, `mistake_screen` renders a mistake screen and other functionality if neccessary.
    ///
    /// Returns `RenderResult<()>`, not `RenderResult<usize>`.
    fn mistake_screen(
        &mut self,
        mut mistakes: Option<Mistakes>,
        card_error: bool,
        just_used_card: Option<Card>,
    ) -> RenderResult<()> {
        let mut all_incs = vec![]; // All "Incorrect use of _."s
        let mut all_fails = vec![]; // All "Failure to say _."s
        let mut card_error_current = true;
        let mut just_used_card = just_used_card;

        loop {
            let (incs, fails) = &mistakes.unwrap_or((vec![], vec![]));
            // println!("BEFORE PRINTING MISTAKES");
            // self.print_mistakes((incs.clone(), fails.clone()));
            // println!("AFTER");

            let n = incs.len()
                + fails.len()
                + (if card_error_current && card_error {
                    1
                } else {
                    0
                });
            if n == 0 {
                // perhaps not necessary?
                // no it's necessary, &mistakes is always Some(([],[])) on successful plays.
                break;
            }

            let mut drawn = self.draw_pile.deal(n, &mut self.players[self.player_index]);
            if card_error {
                if let Some(card) = just_used_card {
                    // self.used_pile
                    self.players[self.player_index].draw(&mut self.used_pile);
                    drawn.insert(0, card);
                }
                just_used_card = None;
            }

            all_incs.extend(incs.iter().map(|v| v.clone()));
            all_fails.extend(fails.iter().map(|v| v.clone()));

            let quotes = self.screen_request_turn_failures(
                all_incs.clone(),
                incs.len(),
                all_fails.clone(),
                fails.len(),
                drawn,
                card_error,
                card_error_current,
            )?;
            card_error_current = false;
            mistakes = self.check_quotes(&quotes, &vec![String::from("thank you")])
        }

        println!("After check");

        Ok(())
    }

    /// A helpful method for debugging purposes for printing the mistakes.
    fn print_mistakes(&self, mistakes: Mistakes) {
        for m in mistakes.0 {
            print!("Incorrect use of {}. ", m);
        }

        print!("\n");

        for m in mistakes.1 {
            print!("Failure to say {}. ", m);
        }

        print!("\n");
    }

    /// Whether a card **about to be played** is a legal play
    /// Must be called before the card enters `self.used_pile`.
    fn validate_card_played(&self, card: Card) -> bool {
        self.used_pile[0].suit().is_red() == card.suit().is_red()
            || self.used_pile[0].value() == card.value()
    }

    /// Utility UI function that requests notifies about change of turn.
    fn screen_next_player(&self) -> RenderResult<()> {
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

    fn parse_action_quotes(data: String) -> Vec<String> {
        data.as_str()
            .split(".")
            .map(|v| v.to_lowercase().trim().to_string())
            .filter(|v| !v.is_empty())
            .collect()
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
        PlayScreen::Turn(TurnState::Speak(did_draw, action_card))
            .render_then_input(Some(self))
            .map(Self::parse_action_quotes)
    }
    /// Utility UI functin that shows their failures when they make them
    /// and allows them to say something in response to getting a card.
    ///
    /// Same format as `self.screen_request_turn_speak`
    ///
    /// Returns `Vec<String>` representing the things they say.
    fn screen_request_turn_failures(
        &self,
        incs: Vec<String>,
        current_incs: usize,
        fails: Vec<String>,
        current_fails: usize,
        drawn: Vec<Card>,
        card_error: bool,
        current_card_error: bool,
    ) -> RenderResult<Vec<String>> {
        PlayScreen::Mistake {
            incs,
            current_incs,
            fails,
            current_fails,
            drawn,
            card_error,
            current_card_error,
        }
        .render_then_input(Some(self))
        .map(Self::parse_action_quotes)
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
    pub fn round_over(&self) -> bool {
        self.round_over
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

#[derive(Debug, Serialize)]
struct Context {
    card: String,
    value: String,
    suit: String,
}

#[derive(Debug)]
pub enum AddingRuleError {
    InvalidAction,
    ConflictingAction,
}

pub type AddingRuleResult<T> = Result<T, AddingRuleError>;

/// A shorthand.
pub type Mistakes = (Vec<String>, Vec<String>);
