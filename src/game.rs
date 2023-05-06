use crate::deck::Deck;
use crate::player::Player;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    draw_pile: Deck,
    used_pile: Deck,
    game_over: bool,
    // rules: Vec<Rule> or HashSet<Rule> ?
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        let mut game = Self {
            players,
            draw_pile: Deck::default_52(),
            used_pile: Deck::empty(),
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

        game
    }

    /// Checks size of `pile`. If it's smaller than `cmp`, then it appends a randomized `Deck`.
    fn check_size_and_append(pile: &mut Deck, cmp: usize) {
        if pile.size() < cmp {
            let mut another = Deck::default_52();
            another.shuffle();
            pile.append(another.into_iter());
        }
    }

    fn players(&self) -> &Vec<Player> {
        &self.players
    }

    fn draw_pile(&self) -> &Deck {
        &self.draw_pile
    }

    fn used_pile(&self) -> &Deck {
        &self.used_pile
    }

    fn game_over(&self) -> bool {
        self.game_over
    }
}
