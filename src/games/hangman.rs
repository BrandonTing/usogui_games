use std::vec;

use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
enum CardType {
    Number,
    Joker,
}

#[derive(Debug)]
struct PlayerCard {
    card_type: CardType,
    number: usize,
}
#[derive(Debug)]
struct Player {
    cards: Vec<PlayerCard>,
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
    players: (Player, Player),
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
        let mut init_cards: Vec<_> = default_cards
            .clone()
            .into_iter()
            .flat_map(|x| {
                vec![
                    PlayerCard {
                        card_type: CardType::Number,
                        number: x,
                    },
                    PlayerCard {
                        card_type: CardType::Number,
                        number: x,
                    },
                ]
            })
            .collect();
        init_cards.push(PlayerCard {
            card_type: CardType::Joker,
            number: init_joker,
        });
        init_cards.shuffle(&mut rng);
        // assign cards to players;
        let index_of_first_action_player: usize = rng.gen_range(0..=1);
        println!(
            "who is going first? player{:?}",
            index_of_first_action_player + 1
        );
        let cards_of_first_player: usize = match index_of_first_action_player {
            0 => default_cards.len() + 1,
            _ => default_cards.len(),
        };
        let player2_cards = init_cards.split_off(cards_of_first_player);
        println!("cards of player1: {:?}", init_cards);
        println!("cards of player2: {:?}", player2_cards);
        return Hangman {
            jokers: default_jokers,
            cards: default_cards,
            required_steps: default_steps,
            players: (
                Player {
                    counter: default_steps,
                    cards: init_cards,
                },
                Player {
                    counter: default_steps,
                    cards: player2_cards,
                },
            ),
        };
    }
}
