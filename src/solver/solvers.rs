use crate::logic::CharMatch;
use std::fmt::Debug;

pub type Guess = String;
pub type SolverWordList<'a> = Vec<&'a str>;

pub trait Solver<'a>
where
    Self: Debug,
{
    fn reload_wordlist(&mut self, wordlist: &SolverWordList<'a>);
    fn guess(&self) -> Option<Guess>;
    fn narrow_words(&mut self, guess_result: &[CharMatch]);

    fn narrow_then_guess(&mut self, guess_result: &[CharMatch]) -> Option<Guess> {
        self.narrow_words(guess_result);
        self.guess()
    }
}
