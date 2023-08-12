use std::vec;

use rand::Rng;

struct PlayerCards {
    card_type: String,
    number: usize,
}

struct Player {
    cards: Vec<PlayerCards>,
    counter: usize,
}

pub struct Hangman {
    // options of numbers on joker card.
    jokers: Vec<usize>,
    // options of numbers on cards.
    cards: Vec<usize>,
    // steps towards HANGMAN
    required_steps: usize,
    // info of each players
    // players: (Player, Player),
}

impl Default for Hangman {
    fn default() -> Self {
        let default_jokers = vec![1, 2, 3, 4, 5];
        let default_cards = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let default_steps: usize = 11;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        let init_joker = default_jokers[rng.gen_range(0..default_jokers.len())];
        println!("joker this round: {}", init_joker);
        // shuffle cards
        let init_cards: Vec<_> = default_cards.into_iter().flat_map(|x| vec![x, x]).collect();

        return Hangman {
            jokers: default_jokers,
            cards: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            required_steps: default_steps,
        };
    }
}
