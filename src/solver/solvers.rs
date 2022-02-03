
type Guess = String;

trait Solver {
	fn load_wordlist(wordlist:HashMap<String>) -> Self;
	fn narrow_guess(&mut self, guess_result: &[CharMatch]) -> Guess;
	fn guess(&self) -> Guess;

	fn narrow_results(&mut self, guess_result: &[CharMatch]);
}
