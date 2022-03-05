use super::types::WordMatch;
use super::*;
use super::{params::GameParameters, state::GameState};
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
    Valid(GuessResult, WordMatch),
}

#[derive(Debug)]
pub enum GameEndTriggers {
    Timeout,
    NoTriesLeft,
    StillPlaying,
}

pub struct Wordle<'a> {
    pub params: GameParameters<'a>,
    pub state: GameState,
}

impl<'a> Wordle<'a> {
    pub fn new_random_game(params: GameParameters<'a>, mut rng: ThreadRng) -> Self {
        let target = params
            .answer_wordlist
            .iter()
            .choose(&mut rng)
            .unwrap()
            .to_string();

        Self::new_game(params, rng, target)
    }

    pub fn check_state(&self) -> GameEndTriggers {
        let time_expired = self
            .params
            .time_limit
            .map(|dur| self.state.start_time.map(|start| dur < start.elapsed()))
            .flatten()
            .unwrap_or(false);

        if time_expired {
            return GameEndTriggers::Timeout;
        }

        let no_tries_left = self
            .params
            .tries
            .map(|x| x >= self.state.prev_guesses.len())
            .unwrap_or(false);

        if no_tries_left {
            return GameEndTriggers::NoTriesLeft;
        }

        GameEndTriggers::StillPlaying
    }

    pub fn new_game(params: GameParameters<'a>, rng: ThreadRng, target: String) -> Self {
        Self {
            state: GameState::new_game(&params, rng, target),
            params,
        }
    }

    pub fn restart(self) -> Self {
        let params = self.params;
        let rng = self.state.rng;

        Self::new_random_game(params, rng)
    }

    pub fn restart_with_target(self, target: String) -> Self {
        let params = self.params;
        let rng = self.state.rng;

        Self::new_game(params, rng, target)
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
        } else if self
            .state
            .prev_guesses
            .iter()
            .any(|old_matches| old_matches.word == guessed_word)
        {
            return WordValidation::Invalid(
                InvalidationReason::RepeatWord,
                guessed_word.to_string(),
            );
        }

        let mut matches = match_word(target_word, guessed_word);
        self.state.prev_guesses.push(matches.clone());

        if *target_word == *guessed_word {
            return WordValidation::Valid(GuessResult::Correct, matches);
        }

        let mutator = &self.params.mutator;

        matches
            .aligns
            .iter_mut()
            .for_each(|x| *x = mutator.mutate(*x, rng));

        WordValidation::Valid(GuessResult::Wrong, matches)
    }
}

pub fn match_word(target: &str, guess: &str) -> WordMatch {
    let (mut target_used, mut matches): (Vec<bool>, Vec<_>) = target
        .chars()
        .zip(guess.chars())
        .map(|(tc, gc)| {
            if tc == gc {
                (true, CharAlignment::Exact)
            } else {
                (false, CharAlignment::NotFound)
            }
        })
        .unzip();

    matches
        .iter_mut()
        .zip(guess.chars())
        .filter(|x| matches!(x.0, CharAlignment::NotFound))
        .for_each(|x| {
            for (has_match, tc) in target_used.iter_mut().zip(target.chars()).filter(|x| !*x.0) {
                if x.1 == tc {
                    *has_match = true;
                    *x.0 = CharAlignment::Misplaced;
                    break;
                }
            }
        });

    WordMatch {
        word: guess.to_string(),
        aligns: matches,
    }
}

impl Default for Wordle<'_> {
    fn default() -> Self {
        let rng = thread_rng();
        let params = GameParameters::default();

        Self::new_random_game(params, rng)
    }
}
