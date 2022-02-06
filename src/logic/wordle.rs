use super::types::StringMatch;
use super::*;
use super::{params::GameParameters, state::GameState, CharMatch};
use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;
use rand::thread_rng;

#[derive(Debug)]
pub enum InvalidationReason {
    WrongLength,
    UnknownWord,
    RepeatWord,
}

#[derive(Debug)]
pub enum GuessResult {
    Correct,
    Wrong,
}

#[derive(Debug)]
pub enum WordValidation {
    Invalid(InvalidationReason, String),
    Valid(GuessResult, Vec<CharMatch>),
}

#[derive(Debug)]
enum GameEndTriggers {
    Timeout,
    NoTriesLeft,
    Finished,
}

pub struct Wordle {
    pub params: GameParameters,
    pub state: GameState,
}

impl Wordle {
    pub fn new_random_game(params: GameParameters, mut rng: ThreadRng) -> Self {
        let target = params
            .answer_wordlist
            .iter()
            .choose(&mut rng)
            .unwrap()
            .to_string();

        Self::new_game(params, rng, target)
    }

    pub fn new_game(params: GameParameters, mut rng: ThreadRng, target: String) -> Self {
        Self {
            state: GameState::new_game(rng, target),
            params,
        }
    }

    pub fn restart(self) -> Self {
        let params = self.params;
        let rng = self.state.rng;

        Self::new_random_game(params, rng)
    }

    pub fn guess(&mut self, guessed_word: &str) -> WordValidation {
        let words = &self.params.guess_wordlist;
        let target_word = self.state.target_word.as_str();
        let range = self.params.word_size;
        let rng = &mut self.state.rng;

        if guessed_word.len() < range.0 || guessed_word.len() > range.1 {
            return WordValidation::Invalid(
                InvalidationReason::WrongLength,
                guessed_word.to_string(),
            );
        } else if !words.contains(guessed_word) {
            return WordValidation::Invalid(
                InvalidationReason::UnknownWord,
                guessed_word.to_string(),
            );
        }

        let matches = match_word(target_word, guessed_word);
        self.state.prev_guesses.push(matches.clone());

        if *target_word == *guessed_word {
            return WordValidation::Valid(GuessResult::Correct, matches);
        }

        let mutator = &self.params.mutator;

        let matches: Vec<CharMatch> = matches
            .into_iter()
            .map(|x| CharMatch {
                align: mutator.mutate(x.align, rng),
                ..x
            })
            .collect();

        WordValidation::Valid(GuessResult::Wrong, matches)
    }
}

fn match_word(target: &str, guess: &str) -> StringMatch {
    target
        .chars()
        .zip(guess.chars())
        .map(|(t, g)| {
            let align = if t == g {
                CharAlignment::Exact
            } else if target.contains(g) {
                CharAlignment::Misplaced
            } else {
                CharAlignment::NotFound
            };

            CharMatch { c: g, align }
        })
        .collect()
}

impl Default for Wordle {
    fn default() -> Self {
        let rng = thread_rng();
        let params = GameParameters::default();

        Self::new_random_game(params, rng)
    }
}
