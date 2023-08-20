mod games;

use games::{
    game::{Game, GameName, GetPlayers},
    hangman::Hangman,
};
use inquire::Select;
use strum::IntoEnumIterator;

fn main() {
    let game_options: Vec<_> = GameName::iter().map(|x| x.to_string()).collect();

    // gen option from game lists
    let game_select = Select::new("choose ur game to play", game_options).prompt();

    let selected_game = match game_select {
        // FIXME is this necessary?
        Ok(game) => {
            if let Some(game_name) = GameName::from_str(&game) {
                match game_name {
                    GameName::Hangman => {
                        println!("new hangman game started");
                        Ok(Game::Hangman(Hangman::default()))
                    }
                }
            } else {
                println!("no matching game");
                Err(())
            }
        }
        Err(err) => {
            println!("Error while getting the game: {}", err);
            Err(())
        }
    };
    let selected_game = selected_game.unwrap();
    let players = selected_game.get_players();
    println!(
        "You are player 1, these are your cards: {:?}",
        players.0.cards
    );
    println!("These are your opponent's cards: {:?}", players.1.cards);
    // TODO Draw cards
}
