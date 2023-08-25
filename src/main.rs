mod games;

use games::{
    game::{Game, GameName},
    hangman::Hangman,
};
use inquire::Select;
use strum::IntoEnumIterator;

use crate::games::game::Play;

fn main() {
    let game_options: Vec<_> = GameName::iter().map(|x| x.to_string()).collect();

    // gen option from game lists
    let game_select = Select::new("choose ur game to play", game_options).prompt();

    let selected_game = match game_select {
        Ok(game) => {
            let game_name = GameName::from_str(&game).unwrap();
            match game_name {
                GameName::Hangman => {
                    println!("new hangman game started");
                    Ok(Game::Hangman(Hangman::default()))
                }
            }
        }
        Err(err) => {
            println!("Error while getting the game: {}", err);
            Err(())
        }
    };
    let mut selected_game = selected_game.unwrap();
    selected_game.play()
}
