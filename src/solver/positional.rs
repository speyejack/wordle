use super::filters::{FilterCriteria, PosFilterCriteria};
use super::solvers::{Guess, Solver, SolverWordList};
use crate::logic::{CharMatch, WordMatch};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PositionalSolver<'a> {
    wordlist: SolverWordList<'a>,
}

impl<'a> PositionalSolver<'a> {
    pub fn new(wordlist: &SolverWordList<'a>) -> Self {
        Self {
            wordlist: wordlist.clone(),
        }
    }
}

impl<'a> Solver<'a> for PositionalSolver<'a> {
    fn reload_wordlist(&mut self, wordlist: &SolverWordList<'a>) {
        self.wordlist = wordlist.clone();
    }

    fn guess(&self) -> Option<Guess> {
        let counts = count_letter(&self.wordlist);
        let word = self
            .wordlist
            .iter()
            .map(|x| (x, score_word(x, &counts)))
            .reduce(|acc, item| if acc.1 > item.1 { acc } else { item })
            .map(|x| x.0.to_string());
        word
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

type LetterCount = HashMap<char, i32>;
type LetterFreq = HashMap<char, f32>;
type PosLetterFreq = Vec<LetterFreq>;

pub fn score_word(word: &str, letter_freq: &PosLetterFreq) -> f32 {
    let char_scores: Vec<(char, f32)> = word
        .chars()
        .zip(letter_freq.iter())
        .map(|(c, local_letter_freq)| {
            let score: f32 = *local_letter_freq.get(&c).unwrap_or(&0.0);
            (c, score)
        })
        .collect();

    let mut letter_map = HashMap::new();
    char_scores.iter().for_each(|(c, score)| {
        let max_score = letter_map.entry(*c).or_insert(0.0);
        *max_score = score.max(*max_score);
    });

    let final_score = letter_map.iter().map(|x| *x.1).sum();
    final_score
}

pub fn count_letter(word_list: &Vec<&'_ str>) -> PosLetterFreq {
    let max_size = word_list.iter().map(|x| x.len()).max().unwrap_or(0);

    let mut pos_count: Vec<LetterCount> = (0..max_size).map(|_| LetterCount::new()).collect();
    let mut tot_count: LetterCount = LetterCount::new();

    for word in word_list.iter() {
        for (c, posmap) in word.chars().zip(pos_count.iter_mut()) {
            *tot_count.entry(c).or_insert(0) += 1;
            *posmap.entry(c).or_insert(0) += 1;
        }
    }

    let tot_weight = 0.1;
    let pos_weight = 1.0;

    let tot_amount = word_list.len() as f32;

    pos_count
        .into_iter()
        .map(|posmap| {
            posmap
                .into_iter()
                .map(|(c, pos_score)| {
                    let pos_freq = pos_score as f32 / tot_amount;
                    let total_freq =
                        *tot_count.entry(c).or_insert(0) as f32 / (tot_amount * max_size as f32);
                    let final_freq = pos_freq * pos_weight + total_freq * tot_weight;
                    (c, final_freq)
                })
                .collect()
        })
        .collect()
}
