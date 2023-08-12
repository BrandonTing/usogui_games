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

fn new_hangman_game(
    joker_options: &Vec<usize>,
    card_options: &Vec<usize>,
    steps: usize,
) -> (Player, Player) {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let init_joker = joker_options[rng.gen_range(0..joker_options.len())];
    println!("joker this round: {}", init_joker);
    // shuffle cards
    let mut init_cards: Vec<_> = card_options
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
        0 => card_options.len() + 1,
        _ => card_options.len(),
    };
    let player2_cards = init_cards.split_off(cards_of_first_player);
    println!("cards of player1: {:?}", init_cards);
    println!("cards of player2: {:?}", player2_cards);
    return (
        Player {
            counter: steps,
            cards: init_cards,
        },
        Player {
            counter: steps,
            cards: player2_cards,
        },
    );
}

impl Default for Hangman {
    fn default() -> Self {
        let default_jokers = vec![1, 2, 3, 4, 5];
        let default_cards = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let default_steps: usize = 11;
        let players = new_hangman_game(&default_jokers, &default_cards, default_steps);

        return Hangman {
            jokers: default_jokers,
            cards: default_cards,
            required_steps: default_steps,
            players: players,
        };
    }
}
