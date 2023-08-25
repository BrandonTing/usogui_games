use std::fmt;

use strum_macros::EnumIter;

use super::hangman::Hangman;

#[derive(EnumIter)]
pub enum GameName {
    Hangman,
}

impl fmt::Display for GameName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameName::Hangman => write!(f, "Hangman"),
        }
    }
}

impl GameName {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Hangman" => Some(Self::Hangman),
            _ => None,
        }
    }
}

pub enum Game {
    Hangman(Hangman),
}

// pub trait GetPlayers {
//     fn get_players(&self) -> (&Player, &Player);
// }

// impl GetPlayers for Game {
//     fn get_players(&self) -> (&Player, &Player) {
//         match self {
//             Game::Hangman(hangman) => {
//                 let (player1, player2) = &hangman.players;
//                 return (player1, player2);
//             }
//         }
//     }
// }

pub trait Play {
    fn play(&mut self) -> ();
}

impl Play for Game {
    fn play(&mut self) -> () {
        match self {
            Game::Hangman(hangman) => hangman.play(),
        }
    }
}
