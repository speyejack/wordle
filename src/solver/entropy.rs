use std::{collections::HashMap, ops::RangeBounds};

use crate::logic::{types::StringMatch, wordle::match_word, CharAlignment, CharMatch};

use super::{
    filters::{FilterCriteria, PosFilterCriteria},
    solvers::{Guess, Solver, SolverWordList},
};

type MatchMap<'a> = HashMap<(&'a str, &'a str), StringMatch>;
type DistMap<'a> = HashMap<&'a str, HashMap<Vec<CharAlignment>, u32>>;

#[derive(Debug)]
pub struct EntropySolver<'a> {
    dist_map: DistMap<'a>,
    wordlist: SolverWordList<'a>,

    prev_guesses: Vec<StringMatch>,
}

impl<'a> EntropySolver<'a> {
    pub fn new(base_wordlist: &SolverWordList<'a>) -> Self {
        let match_map = create_match_map(base_wordlist);
        let dist_map = create_dist_map(base_wordlist, &match_map);

        Self {
            dist_map,
            wordlist: base_wordlist.clone(),
            prev_guesses: Vec::new(),
        }
    }
}

impl<'a> Solver<'a> for EntropySolver<'a> {
    fn reload_wordlist(&mut self, wordlist: &SolverWordList<'a>) {
        self.wordlist = wordlist.clone();
        self.prev_guesses = Vec::new();
    }

    fn guess(&self) -> Option<Guess> {
        let word_total = self.wordlist.len() as f32;
        let prev_guess_strings: Vec<String> = self
            .prev_guesses
            .iter()
            .map(|str_match| str_match.iter().map(|x| x.c).collect::<String>())
            .collect();

        let prev_guess_strs: Vec<&str> = prev_guess_strings.iter().map(|x| x.as_str()).collect();

        let guess = self
            .wordlist
            .iter()
            .filter_map(|guess_word| {
                if prev_guess_strs.contains(guess_word) {
                    return None;
                }

                let prev_entropy: f32 = self
                    .prev_guesses
                    .iter()
                    .map(|x| {
                        let alignment: Vec<CharAlignment> =
                            x.iter().map(|match_char| match_char.align).collect();
                        self.dist_map
                            .get(guess_word)
                            .map(|map| {
                                let local_matches = map.get(&alignment)?;
                                Some((word_total / *local_matches as f32).log2())
                            })
                            .flatten()
                            .unwrap_or(0.0)
                    })
                    .sum();

                let guess_estimated_entropy: f32 = self
                    .dist_map
                    .get(guess_word)
                    .map(|map| {
                        map.values()
                            .map(|v| {
                                let selection_prob = *v as f32 / word_total;
                                let selection_entropy = (1.0 / selection_prob).log2();

                                selection_prob * selection_entropy
                            })
                            .sum()
                    })
                    .unwrap_or(0.0);

                dbg!((guess_word, prev_entropy, guess_estimated_entropy));
                let total_estimated_entropy = prev_entropy / word_total + guess_estimated_entropy;
                // let total_estimated_entropy = guess_estimated_entropy;

                Some((guess_word, total_estimated_entropy))
            })
            .reduce(|best_guess, current_guess| {
                if best_guess.1 >= current_guess.1 {
                    best_guess
                } else {
                    current_guess
                }
            })
            .map(|x| x.0.to_string());

        guess
    }

    fn narrow_words(&mut self, guess_result: &[CharMatch]) {
        self.prev_guesses.push(guess_result.to_vec());
    }
}

fn create_match_map<'a>(wordlist: &SolverWordList<'a>) -> MatchMap<'a> {
    let mut map = HashMap::new();
    for target_word in wordlist {
        for guess_word in wordlist {
            let word_match = match_word(target_word, guess_word);
            map.insert((*target_word, *guess_word), word_match);
        }
    }

    map
}

fn match_potentials<'a>(word: &'a str, str_match: StringMatch) -> Vec<Vec<CharMatch>> {}

fn create_dist_map<'a>(wordlist: &SolverWordList<'a>, match_map: &MatchMap<'a>) -> DistMap<'a> {
    let mut dist_map = HashMap::new();

    for word in wordlist {
        let mut local_dist = HashMap::new();

        for guess_word in wordlist {
            let word_match = match_map.get(&(*word, *guess_word)).unwrap().clone();
            let alignments = word_match.into_iter().map(|x| x.align).collect();

            *local_dist.entry(alignments).or_insert(0) += 1;
        }

        dist_map.insert(*word, local_dist);
    }

    dist_map
}
