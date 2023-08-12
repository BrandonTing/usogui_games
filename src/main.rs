use std::env::args;

use games::hangman::{self, Hangman};

mod games;

fn main() {
    // init new hangman game
    let game = Hangman::default();
}
