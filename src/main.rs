mod games;

use games::{game::Game, hangman::Hangman};
use inquire::Select;
use strum::IntoEnumIterator;

fn main() {
    let mut game_options: Vec<_> = Game::iter().map(|x| x.to_string()).collect();
    let cancel_option = "Actually I don't want to play.";
    game_options.push(cancel_option.to_string());

    // gen option from game lists
    let game_select = Select::new("choose ur game to play", game_options).prompt();

    match game_select {
        Ok(game) => match game {
            _ => {
                if let Some(enum_value) = Game::from_str(&game) {
                    match enum_value {
                        // TODO start a new game
                        Game::Hangman => {
                            let new_game = Hangman::default();
                            println!("new hang game started")
                        }
                    }
                } else if game == cancel_option.to_string() {
                    println!("Player canceled");
                } else {
                    println!("No matching Game found.");
                }
            }
        },
        Err(err) => println!("Error while publishing your status: {}", err),
    }
}
