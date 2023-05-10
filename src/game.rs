use crate::deck::Deck;
use crate::player::Player;
use std::sync::{Mutex, MutexGuard};

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    draw_pile: Deck,
    used_pile: Deck,
    order: Order,
    player_index: usize,
    game_over: bool,
    // rules: Vec<Rule> or HashSet<Rule> ?
}

// SciDev! Please make this description good. Idk if its right.
/// The `Game` singleton that all the player computers reference.
static GAME_SINGLETON: Mutex<Option<Game>> = Mutex::new(None);

impl Game {
    /// Takes a reference to the `Game` singleton in a thread-safe way.
    pub fn instance<'a>() -> MutexGuard<'a, Option<Game>> {
        GAME_SINGLETON.lock().unwrap()
    }

    pub fn begin(players: Vec<Player>) {
        let mut game = Self {
            players,
            draw_pile: Deck::default_52(),
            used_pile: Deck::empty(),
            order: Order::Forward,
            player_index: 0,
            game_over: false,
        };
      

        // Deals 7 cards to each player
        // If there are less than 24 cards left, it adds another 52 cards to the deck
        game.draw_pile.shuffle();
        for player in game.players.iter_mut() {
            game.draw_pile
                .deal(7, player)
                .expect("Should have been able to deal 7 cards to player");
            Game::check_size_and_append(&mut game.draw_pile, 24);
        }

        // Checks for 25 here because 1 card is put in used_pile
        Game::check_size_and_append(&mut game.draw_pile, 25);
        game.draw_pile.inject(1usize, &mut game.used_pile).unwrap();

        // Starts game
        println!("Commence");

        Self::instance() = game;
    }

    /// Checks size of `pile`. If it's smaller than `cmp`, then it appends a randomized `Deck`.
    fn check_size_and_append(pile: &mut Deck, cmp: usize) {
        if pile.size() < cmp {
            let mut another = Deck::default_52();
            another.shuffle();
            pile.append(another.into_iter());
        }
    }

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

        &self.players[self.player_index]
    }

    /// Goes through gameplay loop until a player wins, returns a ref to winning player
    pub fn play(&mut self) -> &Player {
        /*
          Loop through each player
            Player chooses between:
              - playing card to used_pile
                Chooses a card from their hand to play <in Player>
              - taking card from draw_pile
                Draws card from pile <in Player>
        */
        while !self.game_over {
            let choose = true; // true = plays card, false = draws card
            let current = self.current_player_mut();
            if choose {
                if current.num_cards() == 0 {
                    self.game_over = true;
                    break;
                } else if self.current_player().num_cards() == 1 {
                    println!("Mao!");
                }
                current.play_card(0, &mut self.used_pile);
            } else {
                self.current_player_mut().draw(&mut self.draw_pile);
            }

            self.next_player();
        }

        // while !self.game_over {
        //     self.game_over = true;
        // }

        self.current_player()
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.player_index]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.player_index]
    }

    pub fn flip_order(&mut self) {
        self.order = self.order.flip();
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn draw_pile(&self) -> &Deck {
        &self.draw_pile
    }

    pub fn used_pile(&self) -> &Deck {
        &self.used_pile
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

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

#[derive(Debug)]
pub enum Order {
    Forward,
    Backward,
}

impl Order {
    fn flip(&self) -> Order {
        match self {
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }

    fn val(&self) -> isize {
        match self {
            Self::Forward => 1,
            Self::Backward => -1,
        }
    }
}
