use std::{collections::HashMap, vec};

use inquire::Select;
use rand::{seq::SliceRandom, Rng};

use super::game::Play;

#[derive(Debug, Clone, Copy)]
pub enum CardType {
    Number,
    Joker,
}

#[derive(Debug, Clone)]
pub struct PlayerCard {
    pub card_type: CardType,
    pub number: usize,
}
#[derive(Debug)]
pub struct Player {
    pub cards: Vec<PlayerCard>,
    pub counter: usize,
}

pub struct Hangman {
    // options of numbers on joker card.
    pub jokers: Vec<usize>,
    // options of numbers on cards.
    pub cards: Vec<usize>,
    // steps towards HANGMAN
    pub required_steps: usize,
    // index of next attacking player
    pub next_attack_player: usize,
    // info of each players. User will always be player 1
    pub players: (Player, Player),
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
            // Ignore joker
            _ => (),
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
    return result;
}

fn new_hangman_game(
    joker_options: &Vec<usize>,
    card_options: &Vec<usize>,
    index_of_first_action_player: usize,
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

    let cards_of_first_player: usize = match index_of_first_action_player {
        0 => card_options.len() + 1,
        _ => card_options.len(),
    };
    let player2_cards = init_cards.split_off(cards_of_first_player);
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
        // assign cards to players;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        let index_of_first_action_player: usize = rng.gen_range(0..=1);

        match index_of_first_action_player {
            0 => println!("Player will go first"),
            _ => println!("NPC will go first"),
        }

        let (player1_card, player2_card) = new_hangman_game(
            &default_jokers,
            &default_cards,
            index_of_first_action_player,
        );

        return Hangman {
            jokers: default_jokers,
            cards: default_cards,
            required_steps: default_steps,
            next_attack_player: index_of_first_action_player,
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

fn get_values_of_cards(cards: &Vec<PlayerCard>) -> Vec<String> {
    let card_values: Vec<_> = cards
        .into_iter()
        .map(|x| match x.card_type {
            CardType::Joker => String::from("Joker"),
            CardType::Number => x.number.to_string(),
        })
        .collect();
    return card_values;
}

impl Hangman {
    fn draw(&mut self, player_draw: bool, selected_index: usize) {
        let (from, to) = match player_draw {
            true => (&self.players.1.cards, &mut self.players.0.cards),
            false => (&self.players.0.cards, &mut self.players.1.cards),
        };
        println!("from: {:?}", from);
        println!("selected_index: {:?}", selected_index);
        let new_card = from.get(selected_index - 1).unwrap();
        to.push(PlayerCard {
            card_type: new_card.card_type,
            number: new_card.number,
        });

        // filter out the card + shuffle the card;
        let updated_from: Vec<_> = from
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(idx, card)| {
                if idx != selected_index - 1 {
                    Some(card)
                } else {
                    None
                }
            })
            .collect();

        // shuffle again

        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let mut remove_duplicated_from = remove_cards_with_duplication(updated_from.to_vec());
        let mut remove_duplicated_to = remove_cards_with_duplication(to.to_vec());

        // TODO check if finished:
        // check if one has no cards left
        // if no, re play
        // if yes, subtract counts by number of jocker & open new game
        // if counts <= 0, the other win;
        // return play next round
        if remove_duplicated_from.len() == 0 || remove_duplicated_to.len() == 0 {
            //
            println!("game end")
        } else {
            remove_duplicated_from.shuffle(&mut rng);
            remove_duplicated_to.shuffle(&mut rng);

            // update players
            match player_draw {
                true => {
                    self.players = (
                        Player {
                            counter: self.players.0.counter,
                            cards: remove_duplicated_to,
                        },
                        Player {
                            counter: self.players.1.counter,
                            cards: remove_duplicated_from,
                        },
                    );
                }
                false => {
                    self.players = (
                        Player {
                            counter: self.players.0.counter,
                            cards: remove_duplicated_from,
                        },
                        Player {
                            counter: self.players.1.counter,
                            cards: remove_duplicated_to,
                        },
                    );
                }
            }
            println!(
                "Cards player now has: {:?}",
                get_values_of_cards(&self.players.0.cards)
            );
            println!(
                "Cards NPC now has: {:?}",
                get_values_of_cards(&self.players.1.cards)
            );
            match self.next_attack_player {
                0 => self.next_attack_player = 1,
                _ => self.next_attack_player = 0,
            }
            Hangman::play(self);
        }
    }
}

// TODO Draw cards
impl Play for Hangman {
    fn play(&mut self) {
        match self.next_attack_player {
            0 => {
                println!("It's player's turn.");
                println!(
                    "Cards You have: {:?}",
                    get_values_of_cards(&self.players.0.cards)
                );

                let npc_card_options = &self.players.1.cards;
                println!("Cards NPC has: {:?}", get_values_of_cards(npc_card_options));

                let npc_card_index_options = npc_card_options
                    .into_iter()
                    .enumerate()
                    .map(|(idx, _)| idx + 1)
                    .collect();
                // user's turn, show prompt with cards
                let card_select =
                    Select::new("choose the card you want to draw", npc_card_index_options)
                        .prompt();
                match card_select {
                    Ok(card_index) => {
                        println!("Player drawed card {:?}", card_index);
                        Hangman::draw(self, true, card_index);
                    }
                    Err(err) => {
                        println!("Error while getting the game: {}", err);
                    }
                }
            }
            // npc's turn draw random card
            _ => {
                println!("It's NPC's turn.");
                // randomly select 1 card;
                let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
                let selected_card_index = rng.gen_range(0..self.players.0.cards.len());
                Hangman::draw(self, false, selected_card_index + 1);
            }
        }
    }
}
