use crate::logic::CharMatch;
use std::collections::HashSet;

pub type Guess = String;
pub type SolverWordList<'a> = HashSet<&'a str>;

pub trait Solver {
	fn guess(&self) -> Option<Guess>;
	fn narrow_words(&mut self, guess_result: &[CharMatch]);

	fn narrow_then_guess(&mut self, guess_result: &[CharMatch]) -> Option<Guess> {
		self.narrow_words(guess_result);
		self.guess()
	}
}
