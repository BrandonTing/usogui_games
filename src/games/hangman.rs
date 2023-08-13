use std::{collections::HashMap, vec};

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

fn remove_cards_with_duplication(cards: Vec<PlayerCard>) -> Vec<PlayerCard> {
    // create a hashmap to use unique number as key
    let mut count_map: HashMap<usize, usize> = HashMap::new();

    // Count the occurrences of each number
    for card in &cards {
        match card.card_type {
            CardType::Number => {
                let count = count_map.entry(card.number).or_insert(0);
                *count += 1;
            }
            _ => println!("don't count these numbers"),
        }
    }

    // Filter out the numbers that appear more than once
    let result: Vec<_> = cards
        .into_iter()
        .filter(|card| match card.card_type {
            CardType::Number => count_map[&card.number] == 1,
            CardType::Joker => return true,
        })
        .collect();
    println!("{:?}", result);
    return result;
}

fn new_hangman_game(
    joker_options: &Vec<usize>,
    card_options: &Vec<usize>,
) -> (Vec<PlayerCard>, Vec<PlayerCard>) {
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
    println!("cards of player1 before drop duplication: {:?}", init_cards);
    println!("cards of player2: {:?}", player2_cards);
    return (
        remove_cards_with_duplication(init_cards),
        remove_cards_with_duplication(player2_cards),
    );
}

impl Default for Hangman {
    fn default() -> Self {
        let default_jokers = vec![1, 2, 3, 4, 5];
        let default_cards = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let default_steps: usize = 11;
        let (player1_card, player2_card) = new_hangman_game(&default_jokers, &default_cards);

        return Hangman {
            jokers: default_jokers,
            cards: default_cards,
            required_steps: default_steps,
            players: (
                Player {
                    cards: player1_card,
                    counter: default_steps,
                },
                Player {
                    cards: player2_card,
                    counter: default_steps,
                },
            ),
        };
    }
}
