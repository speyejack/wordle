use rand::prelude::IteratorRandom;
use rand::prelude::ThreadRng;
use std::time::Instant;

use super::params::GameParameters;
use super::types::StringMatch;

pub struct GameState {
    pub prev_guesses: Vec<StringMatch>,
    pub target_word: String,
    pub start_time: Instant,
    pub rng: ThreadRng,
}

impl GameState {
    pub fn new_game(params: &GameParameters, mut rng: ThreadRng, target: String) -> Self {
        Self {
            prev_guesses: Vec::new(),
            target_word: target,
            start_time: Instant::now(),
            rng,
        }
    }

    pub fn check_used_word(word: &str) {}
}
