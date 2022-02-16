use std::collections::HashMap;

use crate::logic::{CharMatch, types::matches_str, wordle::match_word, CharAlignment};

use super::{solvers::{SolverWordList, Solver, Guess}, filters::{PosFilterCriteria, FilterCriteria}};

pub struct EntropySolver<'a> {
	wordlist: SolverWordList<'a>,
}

impl<'a> EntropySolver<'a> {
	pub fn load_wordlist(wordlist: SolverWordList<'a>) -> Self {
		Self {
			wordlist,
		}
	}

	fn get_word_entropy(&self, target_word: &str) -> f32{
		let mut probability: HashMap<Vec<CharAlignment>, usize> = HashMap::new();

		let mut total = 0;
		for word in self.wordlist.iter() {
			let char_match = match_word(target_word, word).into_iter().map(|x| x.align).collect();

			*probability.entry(char_match).or_insert(0) += 1;
			total += 1;
		}

		let total = total as f32;

		let entropy:f32 = probability.values().into_iter().map(|x| {
			let match_prob = *x as f32 / total;
			match_prob * (1.0/match_prob).log2()
		}).sum();

		entropy
	}
}

impl<'a> Solver for EntropySolver<'a> {
    fn guess(&self) -> Option<Guess> {
		self.wordlist.iter()
			 .map(|word| (word, self.get_word_entropy(word)))
			 .reduce(|acc, x| if acc.1 < x.1 {x} else {acc})
			 .map(|x| x.0.to_string())
    }

    fn narrow_words(&mut self, guess_result: &[CharMatch]) {
		let filter = PosFilterCriteria::from_matches(guess_result);
		let new_words: SolverWordList = self.wordlist
			.iter()
			.filter_map(|word| Some(word.clone()).filter(|x| filter.check(x)))
			.collect();

		self.wordlist = new_words
    }
}
