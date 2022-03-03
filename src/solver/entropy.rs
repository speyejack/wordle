use super::{
    filters::{FilterCriteria, PosFilterCriteria},
    solvers::{Guess, Solver, SolverWordList},
};
use crate::logic::{types::WordMatch, wordle::match_word, CharAlignment};
use rustc_hash::FxHashMap;
use std::collections::HashMap;

type MatchMap<'a> = FxHashMap<(&'a str, &'a str), WordMatch<'a>>;
type DistMap<'a> = FxHashMap<&'a str, HashMap<Vec<CharAlignment>, u32>>;

#[derive(Debug)]
pub struct EntropySolver<'a> {
    dist_map: DistMap<'a>,
    wordlist: SolverWordList<'a>,

    prev_guesses: Vec<WordMatch<'a>>,
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
        let prev_guess_words: Vec<&str> = self
            .prev_guesses
            .iter()
            .map(|str_match| str_match.word)
            .collect();

        let guess = self
            .wordlist
            .iter()
            .filter_map(|guess_word| {
                if prev_guess_words.contains(guess_word) {
                    return None;
                }

                let prev_entropy: f32 = self
                    .prev_guesses
                    .iter()
                    .map(|x| {
                        let alignment = &x.alignments;
                        self.dist_map
                            .get(guess_word)
                            .map(|map| {
                                let local_matches = map.get(alignment)?;
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

    fn narrow_words(&mut self, guess_result: &WordMatch<'a>) {
        self.prev_guesses.push(guess_result.clone());
    }
}

#[derive(Debug)]
pub struct GlobalShrinkingEntropySolver<'a> {
    match_map: MatchMap<'a>,
    wordlist: SolverWordList<'a>,
}

impl<'a> GlobalShrinkingEntropySolver<'a> {
    pub fn new(base_wordlist: &SolverWordList<'a>) -> Self {
        let match_map = create_match_map(base_wordlist);

        Self {
            match_map,
            wordlist: base_wordlist.clone(),
        }
    }
}

impl<'a> Solver<'a> for GlobalShrinkingEntropySolver<'a> {
    fn reload_wordlist(&mut self, wordlist: &SolverWordList<'a>) {
        self.wordlist = wordlist.clone();
    }

    fn guess(&self) -> Option<Guess> {
        let word_total = self.wordlist.len() as f32;
        let guess = self
            .wordlist
            .iter()
            .filter_map(|guess_word| {
                let word_dist = create_word_dist(guess_word, &self.wordlist, &self.match_map);
                let guess_estimated_entropy: f32 = word_dist
                    .values()
                    .map(|match_count| {
                        let selection_prob = *match_count as f32 / word_total;
                        let selection_entropy = (1.0 / selection_prob).log2();

                        selection_prob * selection_entropy
                    })
                    .sum();

                Some((guess_word, guess_estimated_entropy))
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

    fn narrow_words(&mut self, guess_result: &WordMatch<'a>) {
        let filter = PosFilterCriteria::from_word_match(&guess_result);
        let new_words: SolverWordList = self
            .wordlist
            .iter()
            .filter_map(|word| Some(*word).filter(|x| filter.check(x)))
            .collect();

        self.wordlist = new_words
    }
}

fn create_match_map<'a>(wordlist: &SolverWordList<'a>) -> MatchMap<'a> {
    let mut map = FxHashMap::default();
    for target_word in wordlist {
        for guess_word in wordlist {
            let word_match = match_word(target_word, guess_word);
            map.insert((*target_word, *guess_word), word_match);
        }
    }

    map
}

fn create_word_dist<'a>(
    word: &'a str,
    wordlist: &SolverWordList<'a>,
    match_map: &MatchMap<'a>,
) -> HashMap<Vec<CharAlignment>, u32> {
    let mut word_dist = HashMap::new();

    for guess_word in wordlist {
        let word_match = match_map.get(&(word, *guess_word)).unwrap().clone();
        let alignments = word_match.char_matches().map(|x| x.align).collect();

        *word_dist.entry(alignments).or_insert(0) += 1_u32;
    }

    word_dist
}

fn create_dist_map<'a>(wordlist: &SolverWordList<'a>, match_map: &MatchMap<'a>) -> DistMap<'a> {
    let mut dist_map = DistMap::default();

    for word in wordlist {
        let local_dist = create_word_dist(word, wordlist, match_map);

        dist_map.insert(*word, local_dist);
    }

    dist_map
}

#[derive(Debug)]
pub struct GlobalFilteredEntropySolver<'a> {
    match_map: MatchMap<'a>,
    wordlist: SolverWordList<'a>,
    filtered_list: SolverWordList<'a>,
    prev_guesses: Vec<String>,
}

impl<'a> GlobalFilteredEntropySolver<'a> {
    pub fn new(base_wordlist: &SolverWordList<'a>) -> Self {
        let match_map = create_match_map(base_wordlist);

        Self {
            match_map,
            wordlist: base_wordlist.clone(),
            filtered_list: base_wordlist.clone(),
            prev_guesses: Vec::default(),
        }
    }
}

impl<'a> Solver<'a> for GlobalFilteredEntropySolver<'a> {
    fn reload_wordlist(&mut self, wordlist: &SolverWordList<'a>) {
        self.filtered_list = wordlist.clone();
        self.prev_guesses = Vec::default();
    }

    fn guess(&self) -> Option<Guess> {
        let word_total = self.wordlist.len() as f32;

        let prev_guess_strs: Vec<&str> = self.prev_guesses.iter().map(|x| x.as_str()).collect();

        let guess = self
            .wordlist
            .iter()
            .filter_map(|guess_word| {
                let word_dist = create_word_dist(guess_word, &self.filtered_list, &self.match_map);

                if prev_guess_strs.contains(guess_word) {
                    return None;
                }

                let guess_estimated_entropy: f32 = word_dist
                    .values()
                    .map(|match_count| {
                        let selection_prob = *match_count as f32 / word_total;
                        let selection_entropy = (1.0 / selection_prob).log2();

                        selection_prob * selection_entropy
                    })
                    .sum();

                let prev_multiplier = if self.filtered_list.contains(guess_word) {
                    1.0001
                } else {
                    1.0
                };

                let guess_estimated_entropy = guess_estimated_entropy * prev_multiplier;

                // println!("{:?}", (guess_word, guess_estimated_entropy));
                Some((guess_word, guess_estimated_entropy))
            })
            .reduce(|best_guess, current_guess| {
                if best_guess.1 >= current_guess.1 {
                    best_guess
                } else {
                    current_guess
                }
            })
            .map(|x| x.0.to_string());

        // println!("Guessed: {:?}", guess);

        guess
    }

    fn narrow_words(&mut self, guess_result: &WordMatch<'a>) {
        let filter = PosFilterCriteria::from_word_match(&guess_result);
        let new_words: SolverWordList = self
            .filtered_list
            .iter()
            .filter_map(|word| Some(*word).filter(|x| filter.check(x)))
            .collect();

        self.prev_guesses.push(guess_result.word.to_string());
        self.filtered_list = new_words
    }
}
