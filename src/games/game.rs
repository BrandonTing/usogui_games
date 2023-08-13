use std::fmt;

use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum Game {
    Hangman,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Game::Hangman => write!(f, "Hangman"),
        }
    }
}

impl Game {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Hangman" => Some(Self::Hangman),
            _ => None,
        }
    }
}
