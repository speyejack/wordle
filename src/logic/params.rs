use std::collections::HashSet;
use std::time::Duration;

use super::mutator::{Mutator, NoopMutator};

pub struct GameParameters {
    pub guess_wordlist: HashSet<String>,
    pub answer_wordlist: Vec<String>,
    pub word_size: (usize, usize),

    pub mutator: Box<dyn Mutator>,
    pub tries: Option<i32>,
    pub time_limit: Option<Duration>,
}

impl Default for GameParameters {
    fn default() -> Self {
        let word_size = (5, 5);
        let answer_wordlist: Vec<String> = include_str!("../../words/answers.txt")
            .lines()
            .filter_map(|x| {
                Some(x)
                    .filter(|x| x.len() >= word_size.0 && x.len() <= word_size.1)
                    .map(|x| x.to_string())
            })
            .collect();

        let guess_wordlist = include_str!("../../words/guesses.txt")
            .lines()
            .filter_map(|x| {
                Some(x)
                    .filter(|x| x.len() > word_size.0 && x.len() < word_size.1)
                    .map(|x| x.to_string())
            })
            .chain(answer_wordlist.clone())
            .collect();

        Self {
            guess_wordlist,
            answer_wordlist,
            word_size,

            mutator: Box::new(NoopMutator {}),
            tries: Some(6),
            time_limit: None,
        }
    }
}
