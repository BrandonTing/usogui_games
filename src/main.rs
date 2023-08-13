mod games;
use games::{game::Game, hangman::Hangman};
use inquire::{
    validator::{self, Validation},
    Select, Text,
};

fn main() {
    // // init new hangman game
    // Hangman::default();

    // TODO gen option from game lists
    let game = Select::new(
        "choose ur game to play",
        vec!["Hangman", "Actually I don't want to play."],
    )
    .prompt();

    match game {
        Ok(game) => match game {
            "Actually I don't want to play." => println!("player dropped "),
            _ => println!("The game u choose is: {}", game),
        },
        Err(err) => println!("Error while publishing your status: {}", err),
    }
}
