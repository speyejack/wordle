use std::time::Instant;

use super::mutator::Mutator;
use super::params::GameParameters;
use super::types::WordMatch;

pub struct GameState {
    pub prev_guesses: Vec<WordMatch>,
    pub target_word: String,
    pub start_time: Option<Instant>,
}

impl GameState {
    pub fn new_game<T: Mutator>(params: &GameParameters<T>, target: String) -> Self {
        Self {
            prev_guesses: Vec::new(),
            target_word: target,
            start_time: params.time_limit.map(|_| Instant::now()),
        }
    }
}
